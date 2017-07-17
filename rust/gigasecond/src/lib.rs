extern crate chrono;

use chrono::*;

//31 years, 251 days, 7 hours, 53 minutes, 52 seconds
pub fn after(epoch: DateTime<UTC>) -> DateTime<UTC> {
    return epoch + Duration::seconds(1000000000);
}
