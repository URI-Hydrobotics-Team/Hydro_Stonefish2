#!/usr/bin/env python3
"""
Keyboard teleop for the SimpleVehicle robot defined in sphere_test.scn.

Controls (press a key, no Enter needed):
    w / x   surge forward / backward
    a / d   sway (strafe) left / right
    q / e   yaw left / right (turn in place)
    r / f   rise / dive (vertical)
    SPACE   all stop
    [ / ]   decrease / increase speed scale
    k       quit (also Ctrl+C)

Publishes to /simple_vehicle/thruster_setpoints (std_msgs/Float64MultiArray),
same topic and array order as thruster_control_example.py:
    [T1_FR, T2_FL, T3_RR, T4_RL, T5_VF, T6_VA]

Uses the same allocation (mixing) matrix, derived from the actual thruster
geometry via least-squares pseudo-inverse.
"""

import sys
import select
import termios
import tty

import numpy as np
import rclpy
from rclpy.node import Node
from std_msgs.msg import Float64MultiArray


# Pseudo-inverse allocation matrix: [surge, sway, heave, yaw, pitch] -> 6 thruster setpoints
B_INV = np.array([
    [ 0.354, -0.354,  0.000, -0.786,  0.000],  # T1_FR
    [ 0.354,  0.354,  0.000,  0.786,  0.000],  # T2_FL
    [-0.354, -0.354,  0.000,  0.786,  0.000],  # T3_RR
    [-0.354,  0.354,  0.000, -0.786,  0.000],  # T4_RL
    [ 0.000,  0.000,  0.500,  0.000, -1.000],  # T5_VF
    [ 0.000,  0.000,  0.500,  0.000,  1.000],  # T6_VA
])

# key -> (surge, sway, heave, yaw) delta from neutral, magnitude applied later via speed scale
KEY_CMDS = {
    'w': ( 1,  0,  0,  0),
    'x': (-1,  0,  0,  0),
    'a': ( 0, -1,  0,  0),
    'd': ( 0,  1,  0,  0),
    'q': ( 0,  0,  0, -1),
    'e': ( 0,  0,  0,  1),
    'r': ( 0,  0, -1,  0),   # rise
    'f': ( 0,  0,  1,  0),   # dive
}

HELP = """
SimpleVehicle keyboard teleop
------------------------------
   w/x  surge forward/back      q/e  yaw left/right
   a/d  sway left/right         r/f  rise/dive
   SPACE  all stop              [ / ]  speed down/up
   k or Ctrl+C  quit
------------------------------
"""


def get_key(settings, timeout=0.1):
    """Non-blocking single-character read from stdin (raw terminal mode)."""
    rlist, _, _ = select.select([sys.stdin], [], [], timeout)
    if rlist:
        return sys.stdin.read(1)
    return ''


class TeleopKeyboard(Node):
    def __init__(self):
        super().__init__('teleop_keyboard')
        self.pub = self.create_publisher(
            Float64MultiArray, '/simple_vehicle/thruster_setpoints', 10
        )
        self.speed = 0.5  # 0..1, scales the raw +/-1 command before mixing

    def publish_cmd(self, surge, sway, heave, yaw):
        cmd = np.array([surge, sway, heave, yaw, 0.0]) * self.speed
        setpoints = np.clip(B_INV @ cmd, -1.0, 1.0)
        msg = Float64MultiArray()
        msg.data = [float(v) for v in setpoints]
        self.pub.publish(msg)

    def stop(self):
        self.publish_cmd(0.0, 0.0, 0.0, 0.0)


def main():
    rclpy.init()
    node = TeleopKeyboard()

    settings = termios.tcgetattr(sys.stdin)
    tty.setraw(sys.stdin.fileno())

    print(HELP)
    print(f"Speed scale: {node.speed:.2f}")

    try:
        while True:
            key = get_key(settings)

            if key == '':
                continue
            elif key == '\x03' or key == 'k':  # Ctrl+C or k
                break
            elif key == ' ':
                node.stop()
                print("STOP")
            elif key == '[':
                node.speed = max(0.1, node.speed - 0.1)
                print(f"Speed scale: {node.speed:.2f}")
            elif key == ']':
                node.speed = min(1.0, node.speed + 0.1)
                print(f"Speed scale: {node.speed:.2f}")
            elif key in KEY_CMDS:
                surge, sway, heave, yaw = KEY_CMDS[key]
                node.publish_cmd(surge, sway, heave, yaw)
                print(f"surge={surge} sway={sway} heave={heave} yaw={yaw}  (scale {node.speed:.2f})")
    except KeyboardInterrupt:
        pass
    finally:
        node.stop()
        termios.tcsetattr(sys.stdin, termios.TCSADRAIN, settings)
        node.destroy_node()
        rclpy.shutdown()
        print("\nStopped. Exiting.")


if __name__ == '__main__':
    main()
