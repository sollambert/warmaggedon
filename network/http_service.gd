extends Node

var socket = WebSocketPeer.new();
var http_client = HTTPClient.new();
@export var api_host = "127.0.0.1";
var first_connected = false;

# Called when the node enters the scene tree for the first time.
func _ready():
	http_client.connect_to_host(api_host, 3001)
	create_room();
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	socket.poll();
	var state = socket.get_ready_state();
	if state == WebSocketPeer.STATE_OPEN:
		if !first_connected:
			first_connected = !first_connected;
			on_connected();
		while socket.get_available_packet_count():
			print("Message: ", socket.get_packet().get_string_from_ascii());
	elif state == WebSocketPeer.STATE_CLOSING:
		# do closing, do not break polling
		pass
	elif state == WebSocketPeer.STATE_CLOSED:
		var code = socket.get_close_code();
		var reason = socket.get_close_reason();
		print("WebSocket closed with code: %d, reason %s. Clean: %s" %[code, reason, code != -1]);
		set_process(false);
	pass
	
func create_room():
	var message = ChatHandshake.new("Testing",0,"Dicks").json();
	http_client.request(HTTPClient.METHOD_POST, "http://" + api_host + "/room/create", [], message)

func send_greeting(msg):
	print(msg);
	socket.send_text(msg);
	pass

func on_connected():
	var message = ChatHandshake.new("Testing",0,"Dicks");
	WsService.send_greeting(message.json());
	pass
