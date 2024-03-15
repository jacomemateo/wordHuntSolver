import dbus

if __name__ == "__main__":
    bus = dbus.SystemBus()
    proxy = bus.get_object('org.freedesktop.hostname1','/org/freedesktop/hostname1')
    interface = dbus.Interface(proxy, 'org.freedesktop.DBus.Properties')

    print("---------------")
    hostname = interface.Get('org.freedesktop.hostname1','Hostname')
    print("The host name is ",hostname)


    all_props = interface.GetAll('org.freedesktop.hostname1')
    print(all_props)
