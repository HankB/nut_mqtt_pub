# Network UPS Tool upsc parser

Parse the output from Network UPS Tool (NUT) CLI program `upsc` and write to standard output using JSON format. Typical `upsc` output looks like:

```text
hbarta@oak:~$ upsc EC850LCD@pilog3b
Init SSL without certificate database
battery.charge: 100
battery.charge.low: 10
battery.charge.warning: 20
battery.mfr.date: CPS
battery.runtime: 675
battery.runtime.low: 300
battery.type: PbAcid
battery.voltage: 14.2
battery.voltage.nominal: 12
device.mfr: CPS
device.model: EC850LCD
device.type: ups
driver.flag.ignorelb: enabled
driver.name: usbhid-ups
driver.parameter.bus: 001
driver.parameter.pollfreq: 30
driver.parameter.pollinterval: 15
driver.parameter.port: auto
driver.parameter.product: EC850LCD
driver.parameter.productid: 0501
driver.parameter.synchronous: auto
driver.parameter.vendor: CPS
driver.parameter.vendorid: 0764
driver.version: 2.8.0
driver.version.data: CyberPower HID 0.6
driver.version.internal: 0.47
driver.version.usb: libusb-1.0.26 (API: 0x1000109)
input.transfer.high: 140
input.transfer.low: 96
input.voltage: 123.0
input.voltage.nominal: 120
output.voltage: 122.0
ups.beeper.status: enabled
ups.delay.shutdown: 20
ups.delay.start: 30
ups.load: 40
ups.mfr: CPS
ups.model: EC850LCD
ups.productid: 0501
ups.realpower.nominal: 450
ups.status: OL
ups.test.result: No test initiated
ups.timer.shutdown: -60
ups.timer.start: 0
ups.vendorid: 0764
hbarta@oak:~$ 
```

## Plan

Code something that can parse the output listed above and write desired values in JSON format, including ancillary data such as a timestamp (`time_t`) to standard out. The actual publishing will be delegated to the utility `mosquitto_pub`.

* Selected fields will be hard coded initially. Eventually they could be provided by command line argument or configuratoin file.
* Input will be read from standard in. (IOW, this program will operate as a filter.)

## Similar projects

* `nut2mqtt` <https://github.com/rburkholder/nut2mqtt>
* `nut-to-mqtt` <https://github.com/jnovack/nut-to-mqtt>
* `ups-mqtt` <https://github.com/dniklewicz/ups-mqtt>

Initial plan is to code in Rust.

## Status

Initial version complete.

## Usage

The intended usage for this utility is to execute with input coming from `upsc` piped into this program and output being piped to `mosquitto_pub` on Linux. It should work for either ARM and X86 hosts where these utilities are available. On Debian (including RpiOS) the following packages are required. (The `nut-client` may require configuration as well, but in my case it is run on the host that runs the NUT server so that configuration covers that need.)

```text
apt install mosquitto-clients nut-client
```

The following crontab entry (in a user, not root) crontab publishes the fields of interest every minute.

```text
* * * * * /bin/upsc EC850LCD@localhost | /home/hbarta/bin/nut_upsc_parse | /bin/mosquitto_pub -h mqtt -t HA/"$(/bin/hostname)"/lab/power/EC850LCD@"$(/bin/hostname)" -l
```

## Test

In top level project directory:

```text
cargo build
cat <<EOF | target/debug/nut_upsc_parse  
Init SSL without certificate database
battery.charge: 100
battery.charge.low: 10
battery.charge.warning: 20
battery.mfr.date: CPS
battery.runtime: 675
battery.runtime.low: 300
battery.type: PbAcid
battery.voltage: 14.2
battery.voltage.nominal: 12
device.mfr: CPS
device.model: EC850LCD
device.type: ups
driver.flag.ignorelb: enabled
driver.name: usbhid-ups
driver.parameter.bus: 001
driver.parameter.pollfreq: 30
driver.parameter.pollinterval: 15
driver.parameter.port: autoenter
driver.parameter.product: EC850LCD
driver.parameter.productid: 0501
driver.parameter.synchronous: auto
driver.parameter.vendor: CPS
driver.parameter.vendorid: 0764
driver.version: 2.8.0
driver.version.data: CyberPower HID 0.6
driver.version.internal: 0.47
driver.version.usb: libusb-1.0.26 (API: 0x1000109)
input.transfer.high: 140
input.transfer.low: 96
input.voltage: 123.0
input.voltage.nominal: 120
output.voltage: 122.0
ups.beeper.status: enabled
ups.delay.shutdown: 20
ups.delay.start: 30
ups.load: 40
ups.mfr: CPS
ups.model: EC850LCD
ups.productid: 0501
ups.realpower.nominal: 450
ups.status: OL
ups.test.result: No test initiated
ups.timer.shutdown: -60
ups.timer.start: 0
ups.vendorid: 0764
EOF
```
