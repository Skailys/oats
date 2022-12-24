use std::hash::Hasher;
use std::intrinsics::ctlz;
use base64::alphabet::URL_SAFE;
use base64::encode_engine;
use base64::engine::fast_portable::{FastPortable, NO_PAD};

const ENGINE: FastPortable = FastPortable::from(&URL_SAFE, NO_PAD);

/// A struct that represents an Oat.
pub struct Oat {
    /// The node for the Oat.
    node: u8,
    /// The locally unique identifier for the Oat.
    luid: u64
}

impl Oat {
    /// Creates a new Oat with the given node, sequence number, and timestamp.
    ///
    /// # Assertions
    ///
    /// The `ctlz` function is called on `seq` and `timestamp` with the respective bit lengths.
    /// Both assertions will fail if the result of `ctlz` is not greater than or equal to the bit length minus 12.
    ///
    /// # Examples
    ///
    /// ```
    /// use oat::Oat;
    ///
    /// let oat = Oat::of(1, 0xfff, 0xfffffffffffff);
    /// ```
    pub fn of(node: u8, seq: u16, timestamp: u64) -> Self {
        assert!(ctlz(seq) >= 16 - 12);
        assert!(ctlz(timestamp) >= 64 - 44);

        let luid = (timestamp << 12) | seq as u64;

        Oat {
            node,
            luid
        }
    }
}

impl Oat {
    /// Returns the node for the Oat.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0, 0);
    /// assert_eq!(oat.node(), 1);
    /// ```
    pub fn node(&self) -> u8 {
        self.node
    }

    /// Returns the sequence number for the Oat.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0xfff, 0);
    /// assert_eq!(oat.seq(), 0xfff);
    /// ```
    pub fn seq(&self) -> u16 {
        (self.luid & 0xfff) as u16
    }

    /// Returns the timestamp for the Oat.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0, 0xfffffffffffff);
    /// assert_eq!(oat.timestamp(), 0xfffffffffffff);
    /// ```
    pub fn timestamp(&self) -> u64 {
        self.luid >> 12
    }

    /// Hashes the Oat using the given `Hasher` implementation and returns the result as a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    /// use std::hash::{BuildHasher, Hasher};
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0, 0);
    /// let hasher = RandomState::new().build_hasher();
    /// let hash = oat.hashed(hasher);
    /// ```
    pub fn hashed<H: Hasher>(&self, mut hasher: H) -> String {
        // Write the locally unique identifier to the hasher.
        hasher.write(&self.luid.to_le_bytes());

        // Finish the hash and store the result.
        let hash = hasher.finish();

        // Format the hash and node as a string.
        format!("{:X>2X}{}", &self.node, encode_engine(&hash.to_le_bytes(), &ENGINE))
    }
}

impl ToString for Oat {
    /// Converts the Oat to a string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0, 0);
    /// let string = oat.to_string();
    /// ```
    fn to_string(&self) -> String {
        // Format the locally unique identifier and node as a string.
        format!("{:X>2X}{}", &self.node, encode_engine(&self.luid.to_le_bytes(), &ENGINE))
    }
}