import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource

def generate_launch_description():
    # 1. Locate the stonefish simulator package
    stonefish_launch_dir = os.path.join(
        get_package_share_directory('stonefish_ros2'), 'launch'
    )
    
    # 2. Define the path to your new .scn file
    # (Update this path depending on where you saved it)
    test_sim_share = get_package_share_directory('test_sim') 

    data_path = os.path.join(test_sim_share, 'data')
    scenario_path = os.path.join(test_sim_share, 'scenarios', 'sphere_test.scn')
    # 3. Include the standard simulator and pass arguments
    return LaunchDescription([
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(
                os.path.join(stonefish_launch_dir, 'stonefish_simulator.launch.py')
            ),
            launch_arguments={
                'simulation_data': data_path,
                'scenario_desc': scenario_path,
                'simulation_rate': '300.0',
                'window_res_x': '1024', 
                'window_res_y': '968',
                'rendering_quality': 'high'
            }.items()
        )
    ])

