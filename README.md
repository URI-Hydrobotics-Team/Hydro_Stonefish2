# Hydro_Stonefish2
This repository will hold work for bringing up and setting up Hydro's own Stonefish simulator environment in line with the SOS lab's ROS2 MVP setup.

The primary point of a Stonefish2 setup is to give ourselves simulation for Tadpole and future ROS based AUV's. As the repository is expanded, further information and setup info will be added. Primary topics will include Setup, Setup of .scn files, Properties of .scn files, and how to build a bot in Stonefish.

Directory structure:
`Ros2 Workspace (Can be named anything)`\
                                          |`src` -> Contains ros2 packages you are working on\
                                          |`Dependencies` -> Contains ros2 packages you need for functionality(Make sure to list what is used where for dependencies!)
                                          
Test_Sim:\
A quick testing setup to get Stonefish tested and verify everything is built correctly and working. Features a sphere that drops to the "sea_bed" and a "robot" that has multiple thrusters, can run these thrusters via the keyboard_input.py file in another terminal on your machine. 

Dependencies of Test_Sim: `Stonefish`, `Stonefish_Ros2`, `ROS2 Jazzy`. Each have their own setup steps. Stonefish does not have to stay installed on your system after you do it's setup steps, it is installed like a apt install pkg. ROS2 Jazzy is installed via apt install. Stonefish_Ros2 should be installed in the `Dependencies` folder outside of the `src` folder for workspace seperation. 
