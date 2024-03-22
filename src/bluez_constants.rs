pub const ADAPTER_NAME: &str = "hci0";

pub const BLUEZ_SERVICE_NAME: &str = "org.bluez";
pub const BLUEZ_NAMESPACE: &str = "/org/bluez/";
pub const DBUS_PROPERTIES: &str = "org.freedesktop.DBus.Properties";
pub const DBUS_OM_IFACE: &str = "org.freedesktop.DBus.ObjectManager";

pub const ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";
pub const DEVICE_INTERFACE: &str = "org.bluez.Device1";
pub const GATT_MANAGER_INTERFACE: &str = "org.bluez.GattManager1";
pub const GATT_SERVICE_INTERFACE: &str = "org.bluez.GattService1";
pub const GATT_CHARACTERISTIC_INTERFACE: &str = "org.bluez.GattCharacteristic1";
pub const GATT_DESCRIPTOR_INTERFACE: &str = "org.bluez.GattDescriptor1";
pub const ADVERTISEMENT_INTERFACE: &str = "org.bluez.LEAdvertisement1";
pub const ADVERTISING_MANAGER_INTERFACE: &str = "org.bluez.LEAdvertisingManager1";

pub const RESULT_OK: i32 = 0;
pub const RESULT_ERR: i32 = 1;
pub const RESULT_ERR_NOT_CONNECTED: i32 = 2;
pub const RESULT_ERR_NOT_SUPPORTED: i32 = 3;
pub const RESULT_ERR_SERVICES_NOT_RESOLVED: i32 = 4;
pub const RESULT_ERR_WRONG_STATE: i32 = 5;
pub const RESULT_ERR_ACCESS_DENIED: i32 = 6;
pub const RESULT_EXCEPTION: i32 = 7;
pub const RESULT_ERR_BAD_ARGS: i32 = 8;
pub const RESULT_ERR_NOT_FOUND: i32 = 9;

pub const UUID_NAMES: [(&str, &str); 22] = [
    ("00001801-0000-1000-8000-00805f9b34fb", "Generic Attribute Service"),
    ("0000180a-0000-1000-8000-00805f9b34fb", "Device Information Service"),
    ("e95d93b0-251d-470a-a062-fa1922dfa9a8", "DFU Control Service"),
    ("e95d93af-251d-470a-a062-fa1922dfa9a8", "Event Service"),
    ("e95d9882-251d-470a-a062-fa1922dfa9a8", "Button Service"),
    ("e95d6100-251d-470a-a062-fa1922dfa9a8", "Temperature Service"),
    ("e95dd91d-251d-470a-a062-fa1922dfa9a8", "LED Service"),
    ("00002a05-0000-1000-8000-00805f9b34fb", "Service Changed"),
    ("e95d93b1-251d-470a-a062-fa1922dfa9a8", "DFU Control"),
    ("00002a05-0000-1000-8000-00805f9b34fb", "Service Changed"),
    ("00002a24-0000-1000-8000-00805f9b34fb", "Model Number String"),
    ("00002a25-0000-1000-8000-00805f9b34fb", "Serial Number String"),
    ("00002a26-0000-1000-8000-00805f9b34fb", "Firmware Revision String"),
    ("e95d9775-251d-470a-a062-fa1922dfa9a8", "micro:bit Event"),
    ("e95d5404-251d-470a-a062-fa1922dfa9a8", "Client Event"),
    ("e95d23c4-251d-470a-a062-fa1922dfa9a8", "Client Requirements"),
    ("e95db84c-251d-470a-a062-fa1922dfa9a8", "micro:bit Requirements"),
    ("e95dda90-251d-470a-a062-fa1922dfa9a8", "Button A State"),
    ("e95dda91-251d-470a-a062-fa1922dfa9a8", "Button B State"),
    ("e95d9250-251d-470a-a062-fa1922dfa9a8", "Temperature"),
    ("e95d93ee-251d-470a-a062-fa1922dfa9a8", "LED Text"),
    ("00002902-0000-1000-8000-00805f9b34fb", "Client Characteristic Configuration"),
];

pub const DEVICE_INF_SVC_UUID: &str = "0000180a-0000-1000-8000-00805f9b34fb";
pub const MODEL_NUMBER_UUID: &str = "00002a24-0000-1000-8000-00805f9b34fb";

pub const TEMPERATURE_SVC_UUID: &str = "e95d6100-251d-470a-a062-fa1922dfa9a8";
pub const TEMPERATURE_CHR_UUID: &str = "e95d9250-251d-470a-a062-fa1922dfa9a8";

pub const LED_SVC_UUID: &str = "e95dd91d-251d-470a-a062-fa1922dfa9a8";
pub const LED_TEXT_CHR_UUID: &str = "e95d93ee-251d-470a-a062-fa1922dfa9a8";

