#[macro_use]
extern crate rosrust;


mod msg {
    rosmsg_include!(sensor_msgs/Image,sensor_msgs/Imu,geometry_msgs/TransformStamped);
}


pub fn instantiate_imu_msg() {
    let mut le_msg = msg::sensor_msgs::Imu::default();
    le_msg.linear_acceleration_covariance[0] = 0.0f64;
    println!("orientated ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_expected_msgs() {
        instantiate_imu_msg();
	    assert!(true);
    }
}

