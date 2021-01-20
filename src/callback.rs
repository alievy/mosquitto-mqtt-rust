use crate::model::Message;
use log::{debug, error, info};
use mosquitto_mqtt_sys_rust as mqtt;
use std::ffi::CStr;

const MAGIC_NUMBER: u32 = 0xCA11_BACC;

#[derive(Default)]
pub struct Callbacks {
    magic_number: u32,
    message_callback: Option<Box<dyn Fn(Message)>>,
    connect_callback: Option<Box<dyn Fn(i32)>>,
    disconnect_callback: Option<Box<dyn Fn(i32)>>,
    init: bool,
}

impl Drop for Callbacks {
    fn drop(&mut self) {
        self.magic_number = 0;
        debug!("Dropping callback");
    }
}

impl Callbacks {
    pub fn new() -> Self {
        Callbacks {
            magic_number: MAGIC_NUMBER,
            message_callback: None,
            connect_callback: None,
            disconnect_callback: None,
            init: false,
        }
    }

    /// # Safety
    ///
    /// Initialize user_data.
    pub unsafe fn initialize(&mut self, mosq: *mut mqtt::mosquitto) {
        info!("Initialize user data for mosquitto");
        if !self.init {
            self.init = true;
            let pdata: *const Callbacks = &*self;
            mqtt::mosquitto_user_data_set(mosq, pdata as *mut ::std::os::raw::c_void);
        }
    }

    /// # Safety
    ///
    /// Sets connect callback.
    pub unsafe fn on_connect<C>(&mut self, mosq: *mut mqtt::mosquitto, callback: C)
    where
        C: Fn(i32),
        C: 'static,
    {
        info!("Set connect_callback");
        self.initialize(mosq);
        mqtt::mosquitto_connect_callback_set(mosq, Some(mqtt_connect_callback));
        self.connect_callback = Some(Box::new(callback))
    }

    /// # Safety
    ///
    /// Sets disconnect callback.
    pub unsafe fn on_disconnect<C>(&mut self, mosq: *mut mqtt::mosquitto, callback: C)
    where
        C: Fn(i32),
        C: 'static,
    {
        info!("Set disconnect_callback");
        self.initialize(mosq);
        mqtt::mosquitto_disconnect_callback_set(mosq, Some(mqtt_disconnect_callback));
        self.disconnect_callback = Some(Box::new(callback));
    }

    /// # Safety
    ///
    /// Sets message_callback.
    pub unsafe fn on_message<C>(&mut self, mosq: *mut mqtt::mosquitto, callback: C)
    where
        C: Fn(Message),
        C: 'static,
    {
        self.initialize(mosq);
        mqtt::mosquitto_message_callback_set(mosq, Some(mqtt_message_callback));
        self.message_callback = Some(Box::new(callback));
    }
}

/// # Safety
///
/// C-like mqtt_message_callback.
/// Will be called when broker sends a message.
extern "C" fn mqtt_message_callback(
    _mosq: *mut mqtt::mosquitto,
    data: *mut ::std::os::raw::c_void,
    msg: *const mqtt::mosquitto_message,
) {
    debug!("Recieved MQTT_Message_Callback");
    if msg.is_null() {
        return;
    }

    let this = unsafe { &mut *(data as *mut Callbacks) };

    if this.magic_number != MAGIC_NUMBER {
        error!("Magic number is not valid for disconnect_callback");
        return;
    }

    if let Some(ref callback) = this.message_callback {
        let mqtt_msg = unsafe { &mut *(msg as *mut mqtt::mosquitto_message) };
        let payload = unsafe {
            CStr::from_ptr(mqtt_msg.payload as *const _)
                .to_str()
                .unwrap()
        };
        let topic = unsafe { CStr::from_ptr(mqtt_msg.topic).to_str().unwrap() };
        let message = Message::new(topic, payload);
        callback(message);
    }
}

/// # Safety
///
/// C-like mqtt_connect_callback.
/// Will be called when client is connected to broker.
extern "C" fn mqtt_connect_callback(
    _mosq: *mut mqtt::mosquitto,
    data: *mut ::std::os::raw::c_void,
    response: ::std::os::raw::c_int,
) {
    debug!("Recieved MQTT_Connect_Callback");
    let this = unsafe { &mut *(data as *mut Callbacks) };

    if this.magic_number != MAGIC_NUMBER {
        error!("Magic number is not valid for disconnect_callback");
        return;
    }

    if let Some(ref callback) = this.connect_callback {
        debug!("Notify connect_callback");
        callback(response)
    }
}

/// # Safety
///
/// C-like mqtt_disconnect_callback.Callbacks
/// Will be called when client lose connection to broker.
extern "C" fn mqtt_disconnect_callback(
    _mosq: *mut mqtt::mosquitto,
    data: *mut ::std::os::raw::c_void,
    response: ::std::os::raw::c_int,
) {
    debug!("Recieved MQTT_Disconnect_Callback");
    let this = unsafe { &mut *(data as *mut Callbacks) };

    if this.magic_number != MAGIC_NUMBER {
        error!("Magic number is not valid for disconnect_callback");
        return;
    }

    if let Some(ref callback) = this.disconnect_callback {
        debug!("Notify disconnect_callback");
        callback(response)
    }
}
