use crate::keyboard::KeyCode;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A singular key combination
#[cfg_attr(
    feature = "mirl_derive",
    mirl_derive::derive_all(zerocopy = false, compactly = false)
)]
pub struct KeyBind<T> {
    /// The keys required to activate this keybind besides shift, alt, and control
    pub keys: Vec<KeyCode>,
    /// If shift needs to be pressed for this keybind to be considered pressed (also contributes to its priority)
    pub requires_shift: bool,
    /// If alt needs to be pressed for this keybind to be considered pressed (also contributes to its priority)
    pub requires_alt: bool,
    /// If control needs to be pressed for this keybind to be considered pressed (also contributes to its priority)
    pub requires_control: bool,
    /// The Enum you use for matching the action if the keybind is pressed
    pub action: T,
}
impl<T> KeyBind<T> {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(
        requires_shift: bool,
        requires_alt: bool,
        requires_control: bool,
        keys: Vec<KeyCode>,
        action: T,
    ) -> Self {
        Self {
            keys,
            requires_shift,
            requires_alt,
            requires_control,
            action,
        }
    }
    /// If the conditions for this keybind is met
    #[must_use]
    pub fn is_keybind_activated(
        &self,
        newly_pressed: &[KeyCode],
        shift_pressed: bool,
        alt_pressed: bool,
        control_pressed: bool,
    ) -> bool {
        (self.keys.is_empty() || self.keys.iter().all(|k| newly_pressed.contains(k)))
            && (shift_pressed || !self.requires_shift)
            && (alt_pressed || !self.requires_alt)
            && (control_pressed || !self.requires_control)
    }
    /// Get the priority of a keybind -> Higher priority inputs need to be processed first
    #[must_use]
    pub const fn get_priority(&self) -> usize {
        self.requires_shift as usize
            + self.requires_alt as usize
            + self.requires_control as usize
            + !self.keys.is_empty() as usize
    }
    /// If another keybind has the same or more keys
    #[must_use]
    pub fn does_other_keybind_eat_this_one(&self, other: &Self) -> bool {
        (other.requires_control || !self.requires_control)
            && (other.requires_shift || !self.requires_shift)
            && (other.requires_alt || !self.requires_alt)
            && self.keys.iter().all(|x| other.keys.contains(x))
    }
    /// Remove all keycodes used from the given list
    pub fn remove_required_keys(&self, list: &mut Vec<KeyCode>) {
        list.retain(|x| !self.keys.contains(x));
    }
}

/// If shift + control + left are pressed, shift + left should be discarded
#[must_use]
pub fn sort_actions<T: Clone>(actions: Vec<KeyBind<T>>) -> Vec<KeyBind<T>> {
    let mut actions = actions;

    actions.sort_by_key(KeyBind::get_priority);

    actions.reverse();
    let mut new_actions: Vec<KeyBind<T>> = Vec::new();
    for i in actions.iter().rev() {
        if !actions.iter().any(|y| {
            !core::intrinsics::unlikely(core::ptr::eq(i, y)) // God damn you self checking
                && i.does_other_keybind_eat_this_one(y)
        }) {
            new_actions.push((*i).clone());
        }
    }
    // if !new_actions.is_empty() {
    //     println!("{:#?} \n {}", new_actions, new_actions.len());
    // }

    new_actions
}
#[must_use]
#[allow(clippy::ptr_arg)]
/// Using the available keybinds, newly pressed, and held keys. Determines what keybinds have activated
///
/// Returns: Activated [`KeyBind`]s, remaining [`KeyCode`]s
pub fn handle_keycodes<T: Clone>(
    keybinds: &Vec<KeyBind<T>>,
    newly_pressed: &[KeyCode],
    held: &[KeyCode],
) -> (Vec<KeyBind<T>>, Vec<KeyCode>) {
    let mut newly_pressed = newly_pressed.to_vec();

    let control_down =
        held.contains(&KeyCode::LeftControl) || held.contains(&KeyCode::RightControl);
    let alt_down = held.contains(&KeyCode::LeftAlt) || held.contains(&KeyCode::RightAlt);
    let shift_down = held.contains(&KeyCode::LeftShift) || held.contains(&KeyCode::RightShift);

    let mut active_keybinds = Vec::new();

    for keybind in keybinds.clone() {
        //let k = keybind.keybind();
        let active =
            keybind.is_keybind_activated(&newly_pressed, shift_down, alt_down, control_down);
        if active {
            active_keybinds.push(keybind);
        }
    }
    let new_actions = sort_actions(active_keybinds);

    for i in &new_actions {
        i.remove_required_keys(&mut newly_pressed);
    }
    (new_actions, newly_pressed)
}
