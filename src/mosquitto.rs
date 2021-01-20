use mosquitto_mqtt_sys_rust as mqtt;

use crate::model::Message;
use crate::model::Version;
use crate::Callbacks;
use crate::{Error, Result};
use log::{debug, error, info};
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr;
use std::sync::Once;
use std::u8;

/// Mosquitto
pub struct Mosquitto {
    mosq: *mut mqtt::mosquitto,
    version: Version,
    callback: Option<Callbacks>,
}

unsafe impl std::marker::Send for Mosquitto {}

impl Drop for Mosquitto {
    fn drop(&mut self) {
        info!("Mosquitto: Dropping MQTT");
        self.destroy();
        if let Err(err) = self.cleanup() {
            error!("Could not cleanup MQTT lib: {}", err);
        }
    }
}

impl Mosquitto {
    pub fn new(id: &str) -> Result<Self> {
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            let status = unsafe { mqtt::mosquitto_lib_init() };
            if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
                error!(
                    "Failed to initialize mosquitto lib - status error: {}",
                    status
                );
            }
        });

        let client_id = CString::new(id)?;
        let mosq = unsafe { mqtt::mosquitto_new(client_id.as_ptr(), true, ptr::null_mut()) };
        if mosq.is_null() {
            return Err(Error::MosquittoNull.into());
        }

        Ok(Mosquitto {
            mosq,
            version: Version::obtain_version(),
            callback: None,
        })
    }

    /// Return libmosquitto version.
    pub fn version(&mut self) -> &Version {
        &self.version
    }

    /// Return mosquitto pointer.
    pub fn mosq(&self) -> *mut mqtt::mosquitto {
        self.mosq
    }

    /// Initiate callback.
    pub fn callback_init(&mut self) {
        self.callback = Callbacks::new().into();
    }

    /// Sets message callback.
    pub fn set_message_callback<C>(&mut self, callback: C)
    where
        C: Fn(Message),
        C: 'static,
    {
        if let Some(cb) = &mut self.callback {
            unsafe {
                cb.on_message(self.mosq, callback);
            }
        }
    }

    /// Sets connect callback.
    pub fn set_connect_callback<C>(&mut self, callback: C)
    where
        C: Fn(i32),
        C: 'static,
    {
        if let Some(cb) = &mut self.callback {
            unsafe {
                cb.on_connect(self.mosq, callback);
            }
        }
    }

    /// Sets disconnect callback.
    pub fn set_disconnect_callback<C>(&mut self, callback: C)
    where
        C: Fn(i32),
        C: 'static,
    {
        if let Some(cb) = &mut self.callback {
            unsafe {
                cb.on_disconnect(self.mosq, callback);
            }
        }
    }

    /// Cleanup MQTT
    pub fn cleanup(&mut self) -> Result<()> {
        info!("Mosquitto: mosquitto_lib cleanup");
        let status = unsafe { mqtt::mosquitto_lib_cleanup() };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoCleanup(status).into());
        }
        Ok(())
    }

    /// Destroy MQTT
    pub fn destroy(&self) {
        unsafe { mqtt::mosquitto_destroy(self.mosq) }
    }

    /// Set username and password.
    pub fn set_username_password(&self, username: &str, password: &str) -> Result<()> {
        info!("Mosquitto: Setting username and password");
        let user = CString::new(username)?;
        let password = CString::new(password)?;
        let status =
            unsafe { mqtt::mosquitto_username_pw_set(self.mosq, user.as_ptr(), password.as_ptr()) };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoUserPass(status).into());
        }
        Ok(())
    }

    /// Connect to broker.
    pub fn connect(&mut self, host: &str, port: i32, keep_alive: i32) -> Result<()> {
        info!("Mosquitto: Connect to broker");
        let hostname = CString::new(host)?;
        let status =
            unsafe { mqtt::mosquitto_connect(self.mosq, hostname.as_ptr(), port, keep_alive) };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoConnect(status).into());
        }
        Ok(())
    }

    /// Disconnect to broker.
    pub fn disconnect(&self) -> Result<()> {
        info!("Mosquitto: Disconnect broker");
        let status = unsafe { mqtt::mosquitto_disconnect(self.mosq) };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoDisconnect(status).into());
        }
        Ok(())
    }

    /// Reconnect to broker.
    pub fn reconnect(&self) -> Result<()> {
        debug!("Mosquitto: Reconnect to broker");
        let status = unsafe { mqtt::mosquitto_reconnect(self.mosq) };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoReconnect(status).into());
        }
        Ok(())
    }

    /// Publish to broker.
    pub fn publish(&self, topic: &str, payload: &[u8]) -> Result<()> {
        debug!("Mosquitto: Send publish");
        let tpc = CString::new(topic)?;
        let status = unsafe {
            mqtt::mosquitto_publish(
                self.mosq,
                ptr::null_mut(),
                tpc.as_ptr(),
                payload.len() as ::std::os::raw::c_int,
                payload.as_ptr() as *const ::std::os::raw::c_void,
                0,
                false,
            )
        };

        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoPublish(status).into());
        }
        Ok(())
    }

    /// Subscribe to broker.
    pub fn subscribe(&self, topic: &str) -> Result<()> {
        debug!("Mosquitto: Subscribe to broker");
        let subscription_pattern = CString::new(topic)?;
        let status = unsafe {
            mqtt::mosquitto_subscribe(self.mosq, ptr::null_mut(), subscription_pattern.as_ptr(), 0)
        };

        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoSubscribe(status).into());
        }
        Ok(())
    }

    /// Unsubscribe from broker.
    pub fn unsubscribe(&self, topic: &str) -> Result<()> {
        debug!("Mosquitto: Unsubscribe from broker");
        let subscription_pattern = CString::new(topic)?;
        let status = unsafe {
            mqtt::mosquitto_unsubscribe(self.mosq, ptr::null_mut(), subscription_pattern.as_ptr())
        };

        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoUnsubscribe(status).into());
        }
        Ok(())
    }

    /// MqttLoop
    pub fn mqtt_loop(&self, timeout: i32, maxpackets: i32) -> Result<()> {
        debug!("Mosquitto: mqtt_loop");
        let status = unsafe { mqtt::mosquitto_loop(self.mosq, timeout, maxpackets) };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoMqttLoop(status).into());
        }
        Ok(())
    }

    /// MqttLoopStart. Will create an seperate thread to be running on.
    /// Need to support pthread.
    pub fn mqtt_loop_start(&self) -> Result<()> {
        debug!("Mosquitto: loop_start");
        let status = unsafe { mqtt::mosquitto_loop_start(self.mosq) };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoMqttLoopStart(status).into());
        }
        Ok(())
    }

    /// Mqtt_want_write.
    pub fn loop_want_write(&self) -> bool {
        unsafe { mqtt::mosquitto_want_write(self.mosq) }
    }

    /// Mqtt_loop_write.
    pub fn loop_write(&self, max_packets: i32) -> Result<()> {
        let status = unsafe { mqtt::mosquitto_loop_write(self.mosq, max_packets) };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoLoopWrite(status).into());
        }
        Ok(())
    }

    /// Mqtt_loop_misc.
    pub fn loop_misc(&self) -> Result<()> {
        let status = unsafe { mqtt::mosquitto_loop_misc(self.mosq) };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoLoopMisc(status).into());
        }
        Ok(())
    }

    /// Return socket.
    pub fn socket(&self) -> Result<i32> {
        let status = unsafe { mqtt::mosquitto_socket(self.mosq) };
        if status < 0 {
            return Err(Error::MosquittoSocket.into());
        }
        Ok(status)
    }

    /// Setup TLS encryption.
    pub fn tls_set_using_ca_file(&self, ca_file: &PathBuf) -> Result<()> {
        info!("Mosquitto: Setup TLS");
        let status = unsafe {
            let cert = CString::new(ca_file.to_str().unwrap()).unwrap();
            mqtt::mosquitto_tls_set(
                self.mosq,
                cert.as_ptr(),
                ptr::null() as *const ::std::os::raw::c_char,
                ptr::null() as *const ::std::os::raw::c_char,
                ptr::null() as *const ::std::os::raw::c_char,
                None,
            )
        };
        if status != mqtt::mosq_err_t_MOSQ_ERR_SUCCESS {
            return Err(Error::MosquittoTlsSet(status).into());
        }
        Ok(())
    }
}
