use mosquitto_mqtt_sys_rust as mqtt;

#[derive(Debug)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub revision: i32,
}

impl Version {
    pub fn obtain_version() -> Self {
        let mut major: ::std::os::raw::c_int = 0;
        let mut minor: ::std::os::raw::c_int = 0;
        let mut revision: ::std::os::raw::c_int = 0;

        unsafe {
            mqtt::mosquitto_lib_version(&mut major, &mut minor, &mut revision);
        }

        Version {
            major,
            minor,
            revision,
        }
    }
}
