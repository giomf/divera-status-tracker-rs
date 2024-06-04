# Divera Status Tracker (dst)

## Setup
First, Divera must be set to send status emails to any mailbox at regular intervals.

## Usage
### Fetch all emails

```
dst fetch --host <host> --email <email> --password <password> --subject <subject>  --off-duty-keyword <off-duty-keyword>
```

### Print on-duty status info
```
dst on-duty --print
```

### Export on-duty status info
```
dst on-duty --export
```
