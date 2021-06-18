header = Device - { $name }
device-id = ID: { $id }
device-info = { $info }
button-back = Back
button-edit = Edit
button-fetch = Search
device-stat = { $total ->
        [one] 1 message
        *[other] { $total } messages
    } in total, { $alert ->
        [one] 1 is
        *[other] { $alert } are
    } alert
map-label = Sending places
chart-label = Sending time
start-time-label = Start Time
end-time-label = Start Time
no-data = No message is sent by this device. Maybe this device doesn't exist.
msg-title = Detailed Data
msg-value = Value: { $value }
msg-position = Position: ({ $lng }, { $lat })
msg-time = Time: { $time }
error-label = Failed to fecth data: { $details }
error-net = Net error
error-unknown = Unknown error
error-no-device = Device doesn't exist