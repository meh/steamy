use uinput;
use config::binding::{self, Binding};

macro_rules! button {
	($mapper:expr, $module:ident, $at:expr, $button:expr, $press:expr) => ({
		let events = $mapper.presets.get(&$mapper.preset).unwrap()
			.$module.button(&mut $mapper.device, $at, $button, $press)?;

		if $press {
			for event in events {
				$mapper.pressed.insert(event);
			}
		}
		else {
			for event in events {
				$mapper.pressed.remove(&event);
			}
		}
	});
}

impl<'a> Into<uinput::Event> for &'a Binding {
	fn into(self) -> uinput::Event {
		match self {
			&Binding::Key(ref value) => match value {
				&binding::Key::Esc => uinput::event::keyboard::Key::Esc.into(),

				&binding::Key::F1  => uinput::event::keyboard::Key::F1.into(),
				&binding::Key::F2  => uinput::event::keyboard::Key::F2.into(),
				&binding::Key::F3  => uinput::event::keyboard::Key::F3.into(),
				&binding::Key::F4  => uinput::event::keyboard::Key::F4.into(),
				&binding::Key::F5  => uinput::event::keyboard::Key::F5.into(),
				&binding::Key::F6  => uinput::event::keyboard::Key::F6.into(),
				&binding::Key::F7  => uinput::event::keyboard::Key::F7.into(),
				&binding::Key::F8  => uinput::event::keyboard::Key::F8.into(),
				&binding::Key::F9  => uinput::event::keyboard::Key::F9.into(),
				&binding::Key::F10 => uinput::event::keyboard::Key::F10.into(),
				&binding::Key::F11 => uinput::event::keyboard::Key::F11.into(),
				&binding::Key::F12 => uinput::event::keyboard::Key::F12.into(),

				&binding::Key::BackTick  => uinput::event::keyboard::Key::Grave.into(),
				&binding::Key::_1        => uinput::event::keyboard::Key::_1.into(),
				&binding::Key::_2        => uinput::event::keyboard::Key::_2.into(),
				&binding::Key::_3        => uinput::event::keyboard::Key::_3.into(),
				&binding::Key::_4        => uinput::event::keyboard::Key::_4.into(),
				&binding::Key::_5        => uinput::event::keyboard::Key::_5.into(),
				&binding::Key::_6        => uinput::event::keyboard::Key::_6.into(),
				&binding::Key::_7        => uinput::event::keyboard::Key::_7.into(),
				&binding::Key::_8        => uinput::event::keyboard::Key::_8.into(),
				&binding::Key::_9        => uinput::event::keyboard::Key::_9.into(),
				&binding::Key::_0        => uinput::event::keyboard::Key::_0.into(),
				&binding::Key::Minus     => uinput::event::keyboard::Key::Minus.into(),
				&binding::Key::Equal     => uinput::event::keyboard::Key::Equal.into(),
				&binding::Key::BackSpace => uinput::event::keyboard::Key::BackSpace.into(),

				&binding::Key::Tab          => uinput::event::keyboard::Key::Tab.into(),
				&binding::Key::Q            => uinput::event::keyboard::Key::Q.into(),
				&binding::Key::W            => uinput::event::keyboard::Key::W.into(),
				&binding::Key::E            => uinput::event::keyboard::Key::E.into(),
				&binding::Key::R            => uinput::event::keyboard::Key::R.into(),
				&binding::Key::T            => uinput::event::keyboard::Key::T.into(),
				&binding::Key::Y            => uinput::event::keyboard::Key::Y.into(),
				&binding::Key::U            => uinput::event::keyboard::Key::U.into(),
				&binding::Key::I            => uinput::event::keyboard::Key::I.into(),
				&binding::Key::O            => uinput::event::keyboard::Key::O.into(),
				&binding::Key::P            => uinput::event::keyboard::Key::P.into(),
				&binding::Key::OpenBracket  => uinput::event::keyboard::Key::LeftBrace.into(),
				&binding::Key::CloseBracket => uinput::event::keyboard::Key::RightBrace.into(),

				&binding::Key::CapsLock  => uinput::event::keyboard::Key::CapsLock.into(),
				&binding::Key::A         => uinput::event::keyboard::Key::A.into(),
				&binding::Key::S         => uinput::event::keyboard::Key::S.into(),
				&binding::Key::D         => uinput::event::keyboard::Key::D.into(),
				&binding::Key::F         => uinput::event::keyboard::Key::F.into(),
				&binding::Key::G         => uinput::event::keyboard::Key::G.into(),
				&binding::Key::H         => uinput::event::keyboard::Key::H.into(),
				&binding::Key::J         => uinput::event::keyboard::Key::J.into(),
				&binding::Key::K         => uinput::event::keyboard::Key::K.into(),
				&binding::Key::L         => uinput::event::keyboard::Key::L.into(),
				&binding::Key::SemiColon => uinput::event::keyboard::Key::SemiColon.into(),
				&binding::Key::Quote     => uinput::event::keyboard::Key::Apostrophe.into(),
				&binding::Key::Enter     => uinput::event::keyboard::Key::Enter.into(),

				&binding::Key::LeftShift  => uinput::event::keyboard::Key::LeftShift.into(),
				&binding::Key::Z          => uinput::event::keyboard::Key::Z.into(),
				&binding::Key::X          => uinput::event::keyboard::Key::X.into(),
				&binding::Key::C          => uinput::event::keyboard::Key::C.into(),
				&binding::Key::V          => uinput::event::keyboard::Key::V.into(),
				&binding::Key::B          => uinput::event::keyboard::Key::B.into(),
				&binding::Key::N          => uinput::event::keyboard::Key::N.into(),
				&binding::Key::M          => uinput::event::keyboard::Key::M.into(),
				&binding::Key::Comma      => uinput::event::keyboard::Key::Comma.into(),
				&binding::Key::Dot        => uinput::event::keyboard::Key::Dot.into(),
				&binding::Key::Slash      => uinput::event::keyboard::Key::Slash.into(),
				&binding::Key::RightShift => uinput::event::keyboard::Key::RightShift.into(),

				&binding::Key::LeftControl  => uinput::event::keyboard::Key::LeftControl.into(),
				&binding::Key::Meta         => uinput::event::keyboard::Key::LeftMeta.into(),
				&binding::Key::LeftAlt      => uinput::event::keyboard::Key::LeftAlt.into(),
				&binding::Key::Space        => uinput::event::keyboard::Key::Space.into(),
				&binding::Key::RightAlt     => uinput::event::keyboard::Key::RightAlt.into(),
				&binding::Key::RightControl => uinput::event::keyboard::Key::RightControl.into(),

				&binding::Key::VolumeUp      => uinput::event::keyboard::Misc::VolumeUp.into(),
				&binding::Key::VolumeDown    => uinput::event::keyboard::Misc::VolumeDown.into(),
				&binding::Key::Mute          => uinput::event::keyboard::Misc::Mute.into(),
				&binding::Key::Play          => uinput::event::keyboard::Misc::Play.into(),
				&binding::Key::Stop          => uinput::event::keyboard::Misc::Stop.into(),
				&binding::Key::NextTrack     => uinput::event::keyboard::Misc::NextSong.into(),
				&binding::Key::PreviousTrack => uinput::event::keyboard::Misc::PreviousSong.into(),

				&binding::Key::Insert   => uinput::event::keyboard::Key::Insert.into(),
				&binding::Key::Home     => uinput::event::keyboard::Key::Home.into(),
				&binding::Key::PageUp   => uinput::event::keyboard::Key::PageUp.into(),
				&binding::Key::Delete   => uinput::event::keyboard::Key::Delete.into(),
				&binding::Key::End      => uinput::event::keyboard::Key::End.into(),
				&binding::Key::PageDown => uinput::event::keyboard::Key::PageDown.into(),

				&binding::Key::Up    => uinput::event::keyboard::Key::Up.into(),
				&binding::Key::Down  => uinput::event::keyboard::Key::Down.into(),
				&binding::Key::Left  => uinput::event::keyboard::Key::Left.into(),
				&binding::Key::Right => uinput::event::keyboard::Key::Right.into(),
			},

			&Binding::KeyPad(ref value) => match value {
				&binding::KeyPad::NumLock => uinput::event::keyboard::Key::NumLock.into(),
				&binding::KeyPad::Slash => uinput::event::keyboard::KeyPad::Slash.into(),
				&binding::KeyPad::Asterisk => uinput::event::keyboard::KeyPad::Asterisk.into(),
				&binding::KeyPad::Minus => uinput::event::keyboard::KeyPad::Minus.into(),

				&binding::KeyPad::_7 => uinput::event::keyboard::KeyPad::_7.into(),
				&binding::KeyPad::_8 => uinput::event::keyboard::KeyPad::_8.into(),
				&binding::KeyPad::_9 => uinput::event::keyboard::KeyPad::_9.into(),
				&binding::KeyPad::Plus => uinput::event::keyboard::KeyPad::Plus.into(),

				&binding::KeyPad::_4 => uinput::event::keyboard::KeyPad::_4.into(),
				&binding::KeyPad::_5 => uinput::event::keyboard::KeyPad::_5.into(),
				&binding::KeyPad::_6 => uinput::event::keyboard::KeyPad::_6.into(),

				&binding::KeyPad::_1 => uinput::event::keyboard::KeyPad::_1.into(),
				&binding::KeyPad::_2 => uinput::event::keyboard::KeyPad::_2.into(),
				&binding::KeyPad::_3 => uinput::event::keyboard::KeyPad::_3.into(),
				&binding::KeyPad::Enter => uinput::event::keyboard::KeyPad::Enter.into(),

				&binding::KeyPad::_0 => uinput::event::keyboard::KeyPad::_0.into(),
				&binding::KeyPad::Dot => uinput::event::keyboard::KeyPad::Dot.into(),
			},

			&Binding::XBox(ref value) => match value {
				&binding::XBox::LT => uinput::event::controller::GamePad::TL.into(),
				&binding::XBox::LB => uinput::event::controller::GamePad::TL2.into(),
				&binding::XBox::LS => uinput::event::controller::GamePad::ThumbL.into(),

				&binding::XBox::RT => uinput::event::controller::GamePad::TR.into(),
				&binding::XBox::RB => uinput::event::controller::GamePad::TR2.into(),
				&binding::XBox::RS => uinput::event::controller::GamePad::ThumbR.into(),

				&binding::XBox::X => uinput::event::controller::GamePad::X.into(),
				&binding::XBox::Y => uinput::event::controller::GamePad::Y.into(),
				&binding::XBox::A => uinput::event::controller::GamePad::A.into(),
				&binding::XBox::B => uinput::event::controller::GamePad::B.into(),

				&binding::XBox::Back => uinput::event::controller::GamePad::Select.into(),
				&binding::XBox::Forward => uinput::event::controller::GamePad::Start.into(),

				&binding::XBox::Up => uinput::event::controller::DPad::Up.into(),
				&binding::XBox::Down => uinput::event::controller::DPad::Down.into(),
				&binding::XBox::Left => uinput::event::controller::DPad::Left.into(),
				&binding::XBox::Right => uinput::event::controller::DPad::Right.into(),
			},

			&Binding::Mouse(ref value) => match value {
				&binding::Mouse::Left => uinput::event::controller::Mouse::Left.into(),
				&binding::Mouse::Middle => uinput::event::controller::Mouse::Middle.into(),
				&binding::Mouse::Right => uinput::event::controller::Mouse::Right.into(),

				&binding::Mouse::ScrollUp => uinput::event::relative::Wheel::Vertical.into(),
				&binding::Mouse::ScrollDown => uinput::event::relative::Wheel::Vertical.into(),

				&binding::Mouse::_4 => uinput::event::controller::Mouse::Back.into(),
				&binding::Mouse::_5 => uinput::event::controller::Mouse::Forward.into(),
			},

			&Binding::Action(..) => unreachable!(),
		}
	}
}
