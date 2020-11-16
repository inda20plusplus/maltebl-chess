#[cfg(test)]
mod tests {
	use super::connection_lib::*;

	#[test]
	fn test() {
		macro_rules! assert_message_code {
			($msg:expr, $res:expr) => {{
				let msg = $msg;
				let res = $res;
				assert_eq!(msg.code(), res);
				assert_eq!(Message::from_code(&res).code(), res);
				}};
		}

		// from the protocol

		assert_message_code!(
			Message::Move(Move::Standard {
				origin: Position::new(0, 1),
				target: Position::new(0, 2),
			}),
			[0x01, 0x00, 0x08, 0x10]
		);

		assert_message_code!(
			Message::Move(Move::Promotion {
				origin: Position::new(1, 6),
				target: Position::new(1, 7),
				kind: PieceType::Queen,
			}),
			[0x01, 0x02, 0x31, 0x39, 0x03]
		);

		assert_message_code!(Message::Decline, [0x00]);
	}
}

/// host: player who listens for a connection
/// host: white player, responsible for sending first move message
/// message: has to be minimum size in bytes
/// message: every one declinable with Decline
/// message: on unknown, respond with Decline
pub mod connection_lib {

	pub enum MessageType {
		/// in response to illigal or unwanted action
		Decline,
		/// contains MoveType (as appended byte)
		Move,
		Undo,
		/// in response to Undo or Draw
		Accept,
		/// in response to a Move that results in a checkmate
		Checkmate,
		/// in response to a Move that results in a draw, or to request a draw
		Draw,
		/// in response to a Move in order to resign the match
		Resign,
	}

	pub enum Message {
		Decline,
		Move(Move),
		Undo,
		Accept,
		Checkmate,
		Draw,
		Resign,
	}

	impl Message {
		pub fn get_type(&self) -> MessageType {
			match self {
				Self::Decline => MessageType::Decline,
				Self::Move(_) => MessageType::Move,
				Self::Undo => MessageType::Undo,
				Self::Accept => MessageType::Accept,
				Self::Checkmate => MessageType::Checkmate,
				Self::Draw => MessageType::Draw,
				Self::Resign => MessageType::Resign,
			}
		}
		pub fn code(&self) -> Vec<u8> {
			match self {
				Message::Move(m) => {
					let mut v = m.code();
					v.insert(0, self.get_type().code());
					v
				}
				_ => vec![self.get_type().code()],
			}
		}
		pub fn from_code(code: &[u8]) -> Self {
			let type_code = code.get(0).expect("faulty code for Message");
			let r#type = MessageType::from_code(*type_code);
			match r#type {
				MessageType::Decline => Self::Decline,
				MessageType::Move => Self::Move(Move::from_code(&code[1..])),
				MessageType::Undo => Self::Undo,
				MessageType::Accept => Self::Accept,
				MessageType::Checkmate => Self::Checkmate,
				MessageType::Draw => Self::Draw,
				MessageType::Resign => Self::Resign,
			}
		}
	}

	impl MessageType {
		pub fn code(&self) -> u8 {
			match self {
				Self::Decline => 0x00,
				Self::Move => 0x01,
				Self::Undo => 0x02,
				Self::Accept => 0x03,
				Self::Checkmate => 0x04,
				Self::Draw => 0x05,
				Self::Resign => 0x06,
			}
		}
		pub fn from_code(code: u8) -> Self {
			match code {
				0x00 => Self::Decline,
				0x01 => Self::Move,
				0x02 => Self::Undo,
				0x03 => Self::Accept,
				0x04 => Self::Checkmate,
				0x05 => Self::Draw,
				0x06 => Self::Resign,
				_ => panic!("falty code for MessageType"),
			}
		}
	}

	pub enum MoveType {
		Standard,
		EnPassant,
		Promotion,
		CastleKingside,
		CastleQueenside,
	}

	pub struct Position {
		pub x: u8,
		pub y: u8,
	}

	impl Position {
		pub fn new(x: u8, y: u8) -> Self {
			Self { x, y }
		}
		pub fn code(&self) -> u8 {
			(self.x & 0b111u8) | ((self.y << 3) & (0b111000u8))
		}
		pub fn from_code(code: u8) -> Self {
			Self {
				x: code & 0b111u8,
				y: (code >> 3) & (0b111u8),
			}
		}
	}

	pub enum Move {
		Standard {
			origin: Position,
			target: Position,
		},
		EnPassant {
			origin: Position,
			target: Position,
		},
		Promotion {
			origin: Position,
			target: Position,
			kind: PieceType,
		},
		CastleKingside,
		CastleQueenside,
	}

	impl Move {
		pub fn get_type(&self) -> MoveType {
			match self {
				Move::Standard {
					origin: _,
					target: _,
				} => MoveType::Standard,
				Move::EnPassant {
					origin: _,
					target: _,
				} => MoveType::EnPassant,
				Move::Promotion {
					origin: _,
					target: _,
					kind: _,
				} => MoveType::Promotion,
				Move::CastleKingside => MoveType::CastleKingside,
				Move::CastleQueenside => MoveType::CastleQueenside,
			}
		}
		pub fn code(&self) -> Vec<u8> {
			let mut data = vec![self.get_type().code()];
			match self {
				Move::Standard { origin, target } => {
					data.push(origin.code());
					data.push(target.code());
				}
				Move::EnPassant { origin, target } => {
					data.push(origin.code());
					data.push(target.code());
				}
				Move::Promotion {
					origin,
					target,
					kind,
				} => {
					data.push(origin.code());
					data.push(target.code());
					data.push(kind.code());
				}
				Move::CastleKingside => (),
				Move::CastleQueenside => (),
			};
			data
		}
		pub fn from_code(code: &[u8]) -> Self {
			let type_code = code.get(0).expect("faulty code for Move");
			let r#type = MoveType::from_code(*type_code);
			match r#type {
				MoveType::Standard => Self::Standard {
					origin: Position::from_code(*code.get(1).expect("faulty code for Move, expected origin")),
					target: Position::from_code(*code.get(2).expect("faulty code for Move, expected target")),
				},
				MoveType::EnPassant => Self::EnPassant {
					origin: Position::from_code(*code.get(1).expect("faulty code for Move, expected origin")),
					target: Position::from_code(*code.get(2).expect("faulty code for Move, expected target")),
				},
				MoveType::Promotion => Self::Promotion {
					origin: Position::from_code(*code.get(1).expect("faulty code for Move, expected origin")),
					target: Position::from_code(*code.get(2).expect("faulty code for Move, expected target")),
					kind: PieceType::from_code(*code.get(3).expect("faulty code for Move, expected kind")),
				},
				MoveType::CastleKingside => Self::CastleKingside,
				MoveType::CastleQueenside => Self::CastleQueenside,
			}
		}
	}

	impl MoveType {
		pub fn code(&self) -> u8 {
			match self {
				Self::Standard => 0x00,
				Self::EnPassant => 0x01,
				Self::Promotion => 0x02,
				Self::CastleKingside => 0x03,
				Self::CastleQueenside => 0x04,
			}
		}
		pub fn from_code(code: u8) -> Self {
			match code {
				0x00 => Self::Standard,
				0x01 => Self::EnPassant,
				0x02 => Self::Promotion,
				0x03 => Self::CastleKingside,
				0x04 => Self::CastleQueenside,
				_ => panic!("invalid code for MoveType"),
			}
		}
	}

	pub enum PieceType {
		Knight,
		Bishop,
		Rook,
		Queen,
	}

	impl PieceType {
		pub fn code(&self) -> u8 {
			match self {
				Self::Knight => 0x00,
				Self::Bishop => 0x01,
				Self::Rook => 0x02,
				Self::Queen => 0x03,
			}
		}
		pub fn from_code(code: u8) -> Self {
			match code {
				0x00 => Self::Knight,
				0x01 => Self::Bishop,
				0x02 => Self::Rook,
				0x03 => Self::Queen,
				_ => panic!("invalid code for PieceType"),
			}
		}
	}
}
