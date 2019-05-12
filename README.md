rosbag_source

Allows reading an ordered series of ROS messages from a ROS Bag file. 


## What's in your bag file?

Assuming you have the ROS command line tools available, you can run eg:

`rosbag info dataset-room1_512_16.bag`

and get a summary of what's in the bag file, such as:

```
path:        dataset-room1_512_16.bag
version:     2.0
duration:    2:21s (141s)
start:       Mar 08 2018 17:31:48.18 (1520530308.18)
end:         Mar 08 2018 17:34:09.23 (1520530449.23)
size:        2.8 GB
messages:    50305
compression: none [57/57 chunks]
types:       geometry_msgs/TransformStamped [b5764a33bfeb3588febc2682852579b0]
             sensor_msgs/Image              [060021388200f6f0f447d0fcd9c64743]
             sensor_msgs/Imu                [6a62c6daae103f4ff57a132d6f95cec2]
topics:      /cam0/image_raw               2821 msgs    : sensor_msgs/Image             
             /cam1/image_raw               2821 msgs    : sensor_msgs/Image             
             /imu0                        28122 msgs    : sensor_msgs/Imu               
             /vrpn_client/raw_transform   16541 msgs    : geometry_msgs/TransformStamped
```