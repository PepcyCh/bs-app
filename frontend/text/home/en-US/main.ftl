header = Home
welcome = Welcome, { $username }({ $email })!
id-label = Device ID
id-hint = Device ID to be added
button-add = Add Device
button-fetch = Refresh Devices
button-logout = Logout
button-edit = Edit
button-details = Details
button-delete = Delete
device-stat = { $total ->
        [one] 1 message
        *[other] { $total } messages
    }, { $alert ->
        [one] 1 is
        *[other] { $alert } are
    } alert
error-label = Error: { $details }
error-net = Net error
error-unknown = Unknown error
error-no-device = Device doesn't exist
error-no-user = User doesn't exist