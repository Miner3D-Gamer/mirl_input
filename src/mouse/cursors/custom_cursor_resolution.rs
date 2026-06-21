// use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Resolution settings for cursor related tasks
pub struct CustomCursorResolution<Type> {
    /// [`CursorResolution::X16`]
    pub x16: Type,
    /// [`CursorResolution::X32`]
    pub x32: Type,
    /// [`CursorResolution::X48`]
    pub x48: Type,
    /// [`CursorResolution::X64`]
    pub x64: Type,
    /// [`CursorResolution::X96`]
    pub x96: Type,
    /// [`CursorResolution::X128`]
    pub x128: Type,
    /// [`CursorResolution::X256`]
    pub x256: Type,
}
impl<Type> CustomCursorResolution<Type> {
    /// Take all values and return them in a [`Vec`]
    pub fn to_vec(self) -> Vec<Type> {
        vec![
            self.x16, self.x32, self.x48, self.x64, self.x96, self.x128,
            self.x256,
        ]
    }
    /// Take all values and return a ref to them in a [`Vec`]
    pub fn to_vec_ref(&self) -> Vec<&Type> {
        vec![
            &self.x16, &self.x32, &self.x48, &self.x64, &self.x96, &self.x128,
            &self.x256,
        ]
    }
}
impl<Type> CustomCursorResolution<Option<Type>> {
    /// Take all values that aren't [`None`] and return them in a [`Vec`]. If all values are [`None`], return [`None`]
    pub fn to_non_empty_vec(self) -> Option<Vec<Type>> {
        let list = self.to_vec();
        let mut output = Vec::with_capacity(7);
        for i in list.into_iter().flatten() {
            output.push(i);
        }
        if output.is_empty() {
            None
        } else {
            Some(output)
        }
    }
    /// Take all values that aren't [`None`] and return them in a [`Vec`]. If all values are [`None`], return [`None`]
    pub fn to_non_empty_vec_ref(&self) -> Option<Vec<&Type>> {
        let list = self.to_vec_ref();
        let mut output = Vec::with_capacity(7);
        for i in list.into_iter().flatten() {
            output.push(i);
        }
        if output.is_empty() {
            None
        } else {
            Some(output)
        }
    }
    /// Get the type in the lowest the lowest resolution available
    ///
    /// ---
    ///
    /// For the highest resolution use [`get_highest_resolution`](Self::get_highest_resolution)
    ///
    /// To get all available cursors use [`to_non_empty_vec_ref`](Self::to_non_empty_vec_ref)
    pub const fn get_lowest_resolution(&self) -> Option<&Type> {
        if let Some(c) = &self.x16 {
            return Some(c);
        }
        if let Some(c) = &self.x32 {
            return Some(c);
        }
        if let Some(c) = &self.x48 {
            return Some(c);
        }
        if let Some(c) = &self.x64 {
            return Some(c);
        }
        if let Some(c) = &self.x96 {
            return Some(c);
        }
        if let Some(c) = &self.x128 {
            return Some(c);
        }
        if let Some(c) = &self.x256 {
            return Some(c);
        }
        None
    }
    /// Get the type in the lowest the lowest resolution available
    ///
    /// ---
    ///
    /// For the lowest resolution use [`get_lowest_resolution`](Self::get_lowest_resolution)
    ///
    /// To get all available cursors use [`to_non_empty_vec_ref`](Self::to_non_empty_vec_ref)
    pub const fn get_highest_resolution(&self) -> Option<&Type> {
        if let Some(c) = &self.x256 {
            return Some(c);
        }
        if let Some(c) = &self.x128 {
            return Some(c);
        }
        if let Some(c) = &self.x96 {
            return Some(c);
        }
        if let Some(c) = &self.x64 {
            return Some(c);
        }
        if let Some(c) = &self.x48 {
            return Some(c);
        }
        if let Some(c) = &self.x32 {
            return Some(c);
        }
        if let Some(c) = &self.x16 {
            return Some(c);
        }
        None
    }
}
impl<T> core::default::Default for CustomCursorResolution<Option<T>> {
    default fn default() -> Self {
        Self {
            x16: None,
            x32: None,
            x48: None,
            x64: None,
            x96: None,
            x128: None,
            x256: None,
        }
    }
}
