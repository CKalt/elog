# elog
elog is a simple REST server that accepts messages and logs them to
a file name log-file.

# endpoints
 - The messages are received as POSTs to the endpoint: http::/localhost:3000/message.
 - A web page for sending messages can be accessed via: http://localhost:3000/

# message format
Messages are accepted as a very simple JSON Object with a single member
"message" that contains the message text.
{
    message: "Message Text"
}

# Using Web (Browser) Clients
elog uses a CORS layer (from tower_http crate) so that cross site API access
is possible.


For this index.html can be served from origins with differnet hosts
from which the elog endpoint is available.
