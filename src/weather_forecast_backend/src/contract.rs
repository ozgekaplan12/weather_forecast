#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;
use ink_prelude::string::String;
use ink_prelude::vec::Vec;
use ink_storage::{
    lazy::Lazy,
    traits::{
        PackedLayout,
        SpreadLayout,
    },
};

#[ink::contract]
mod weather_contract {
    use super::*;

    /// Struct to represent the weather state
    #[derive(Default, Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct WeatherState {
        temperature: i32,
        pressure: i32,
        humidity: i32,
        wind_speed: i32,
    }

    /// Thread-local storage to maintain state across contract upgrades
    thread_local! {
        static WEATHER_STATE: Lazy<WeatherState> = Lazy::new(Default::default);
    }

    /// Define the main contract functionality
    #[ink(storage)]
    pub struct WeatherContract {
        state: Lazy<WeatherState>,
    }

    impl WeatherContract {
        /// Constructor initializes the weather state
        #[ink(constructor)]
        pub fn new(initial_temperature: i32, initial_pressure: i32, initial_humidity: i32, initial_wind_speed: i32) -> Self {
            let mut weather_state = WeatherState::default();
            weather_state.temperature = initial_temperature;
            weather_state.pressure = initial_pressure;
            weather_state.humidity = initial_humidity;
            weather_state.wind_speed = initial_wind_speed;

            Self {
                state: Lazy::new(weather_state),
            }
        }

        /// Set the weather data
        #[ink(message)]
        pub fn set_weather_data(&mut self, temperature: i32, pressure: i32, humidity: i32, wind_speed: i32) {
            self.state.execute_mut(|state| {
                state.temperature = temperature;
                state.pressure = pressure;
                state.humidity = humidity;
                state.wind_speed = wind_speed;
            });
        }

        /// Get the current temperature
        #[ink(message)]
        pub fn get_temperature(&self) -> i32 {
            self.state.execute_read(|state| state.temperature).unwrap_or_default()
        }

        /// Get the current pressure
        #[ink(message)]
        pub fn get_pressure(&self) -> i32 {
            self.state.execute_read(|state| state.pressure).unwrap_or_default()
        }

        /// Get the current humidity
        #[ink(message)]
        pub fn get_humidity(&self) -> i32 {
            self.state.execute_read(|state| state.humidity).unwrap_or_default()
        }

        /// Get the current wind speed
        #[ink(message)]
        pub fn get_wind_speed(&self) -> i32 {
            self.state.execute_read(|state| state.wind_speed).unwrap_or_default()
        }

        /// Update temperature
        #[ink(message)]
        pub fn update_temperature(&mut self, new_temperature: i32) {
            self.state.execute_mut(|state| {
                state.temperature = new_temperature;
            });
        }

        /// Update pressure
        #[ink(message)]
        pub fn update_pressure(&mut self, new_pressure: i32) {
            self.state.execute_mut(|state| {
                state.pressure = new_pressure;
            });
        }

        /// Update humidity
        #[ink(message)]
        pub fn update_humidity(&mut self, new_humidity: i32) {
            self.state.execute_mut(|state| {
                state.humidity = new_humidity;
            });
        }

        /// Update wind speed
        #[ink(message)]
        pub fn update_wind_speed(&mut self, new_wind_speed: i32) {
            self.state.execute_mut(|state| {
                state.wind_speed = new_wind_speed;
            });
        }
    }
}
