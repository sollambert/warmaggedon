extends Node

var socket = WebSocketPeer.new();
@export var ws_base_url = "ws://127.0.0.1:3001";
var first_connected = false;

# Called when the node enters the scene tree for the first time.
func _ready():
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
	
func join_room(id):
	socket.connect_to_url(ws_base_url + "/room/join/" + id)

func send_message(msg):
	print(msg);
	socket.send_text(msg);
	pass

func on_connected():
	var message = ChatHandshake.new("Testing",0,"Dicks");
	WsService.send_greeting(message.json());
	pass
