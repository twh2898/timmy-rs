
extern crate chrono;
use chrono::{DateTime, Utc};

pub mod prelude {
    pub use Timed;
    pub use TSData;
}

pub fn Timed<D>(data: D) -> TSData<D> {
    TSData::new(data)
}

pub struct TSData<D> {
    data: D,
    timestamp: DateTime<Utc>,
}

impl<D> TSData<D> {
    pub fn new(data: D) -> Self {
        TSData {
            data: data,
            timestamp: Utc::now(),
        }
    }

    pub fn with_timestamp(self, timestamp: DateTime<Utc>) -> Self {
        TSData {
            timestamp: timestamp,
            ..self
        }
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
