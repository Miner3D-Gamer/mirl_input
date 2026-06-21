#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
// #[cfg_attr(feature = "wincode", derive(wincode::SchemaWrite, wincode::SchemaRead))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// The resolution/size of the given cursor
///
/// If a given resolution is not supported, the next smaller is tried
pub enum CursorResolution {
    /// 16x16 - Low-resolution legacy or fallback
    X16,
    #[default]
    /// 32x32 - 100% Scaling (96 DPI)
    X32,
    /// 48x48 - 150% Scaling (144 DPI)
    X48,
    /// 64x64 - 200% Scaling (192 DPI)
    X64,
    /// 96x96 - 300% Scaling (288 DPI)
    X96,
    /// 128x128 - 400% Scaling or Accessibility Extra Large
    X128,
    /// 256x256
    X256,
}

impl CursorResolution {
    #[must_use]
    /// Get the mathematical resolution size
    pub const fn get_size(&self) -> u16 {
        u16::from(self.get_size_pos()) + 1
    }
    #[must_use]
    /// Get the resolution position - Where 0 is 1 or Logical size - 1
    pub const fn get_size_pos(&self) -> u8 {
        match self {
            Self::X16 => 15,
            Self::X32 => 31,
            Self::X48 => 47,
            Self::X64 => 63,
            Self::X96 => 95,
            Self::X128 => 127,
            Self::X256 => 255,
        }
    }
    /// Check if given size is valid for a cursor
    #[must_use]
    pub fn is_valid_size<T: TryInto<u16> + PartialEq>(res: (T, T)) -> bool {
        if res.0 != res.1 {
            return false;
        }
        let Ok(n) = res.0.try_into() else {
            return false;
        };
        Self::from_resolution(n).is_some()
    }
    /// Check if given size is valid for a cursor - 1
    #[must_use]
    pub fn is_valid_pos_size<T: TryInto<u8> + PartialEq>(res: (T, T)) -> bool {
        if res.0 != res.1 {
            return false;
        }
        let Ok(n) = res.0.try_into() else {
            return false;
        };
        Self::from_pos_resolution(n).is_some()
    }
    /// Create a [`CursorResolution`] if the input size is valid (Max: 256)
    #[must_use]
    pub fn from_sized_resolution<T: TryInto<u16> + PartialEq>(
        size: (T, T),
    ) -> Option<Self> {
        if size.0 != size.1 {
            return None;
        }
        let Ok(n) = size.0.try_into() else {
            return None;
        };
        Self::from_resolution(n)
    }
    /// Create a [`CursorResolution`] if the input size is valid (Max: 255)
    #[must_use]
    pub fn from_sized_pos_resolution<T: TryInto<u8> + PartialEq>(
        size: (T, T),
    ) -> Option<Self> {
        if size.0 != size.1 {
            return None;
        }
        let Ok(n) = size.0.try_into() else {
            return None;
        };
        Self::from_pos_resolution(n)
    }
    #[must_use]
    /// Get the cursor resolution from a [`u16`] if the inputted resolution is unsupported [`None`] is returned
    pub const fn from_resolution(size: u16) -> Option<Self> {
        Some(match size {
            16 => Self::X16,
            32 => Self::X32,
            48 => Self::X48,
            64 => Self::X64,
            96 => Self::X96,
            128 => Self::X128,
            256 => Self::X256,
            _ => return None,
        })
    }
    #[must_use]
    /// Get the cursor resolution from a [`u8`] if the inputted resolution is unsupported [`None`] is returned
    pub const fn from_pos_resolution(size: u8) -> Option<Self> {
        Some(match size {
            15 => Self::X16,
            31 => Self::X32,
            47 => Self::X48,
            63 => Self::X64,
            95 => Self::X96,
            127 => Self::X128,
            255 => Self::X256,
            _ => return None,
        })
    }
}
