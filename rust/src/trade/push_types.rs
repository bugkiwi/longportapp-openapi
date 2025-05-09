use std::str::FromStr;

use longport_proto::trade::Notification;
use prost::Message;
use rust_decimal::Decimal;
use serde::Deserialize;
use strum_macros::{Display, EnumString};
use time::OffsetDateTime;

use crate::{
    Error, Result, serde_utils,
    trade::{OrderSide, OrderStatus, OrderTag, OrderType, TriggerStatus, cmd_code},
};

/// Topic type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum TopicType {
    /// Private notification for trade
    #[strum(serialize = "private")]
    Private,
}

/// Order changed message
#[derive(Debug, Deserialize)]
pub struct PushOrderChanged {
    /// Order side
    pub side: OrderSide,
    /// Stock name
    pub stock_name: String,
    /// Submitted quantity
    pub submitted_quantity: Decimal,
    /// Order symbol
    pub symbol: String,
    /// Order type
    pub order_type: OrderType,
    /// Submitted price
    pub submitted_price: Decimal,
    /// Executed quantity
    pub executed_quantity: Decimal,
    /// Executed price
    #[serde(with = "serde_utils::decimal_opt_0_is_none")]
    pub executed_price: Option<Decimal>,
    /// Order ID
    pub order_id: String,
    /// Currency
    pub currency: String,
    /// Order status
    pub status: OrderStatus,
    /// Submitted time
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "serde_utils::timestamp::deserialize"
    )]
    pub submitted_at: OffsetDateTime,
    /// Last updated time
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "serde_utils::timestamp::deserialize"
    )]
    pub updated_at: OffsetDateTime,
    /// Order trigger price
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub trigger_price: Option<Decimal>,
    /// Rejected message or remark
    pub msg: String,
    /// Order tag
    pub tag: OrderTag,
    /// Conditional order trigger status
    #[serde(with = "serde_utils::trigger_status")]
    pub trigger_status: Option<TriggerStatus>,
    /// Conditional order trigger time
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub trigger_at: Option<OffsetDateTime>,
    /// Trailing amount
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub trailing_amount: Option<Decimal>,
    /// Trailing percent
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub trailing_percent: Option<Decimal>,
    /// Limit offset amount
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub limit_offset: Option<Decimal>,
    /// Account no
    pub account_no: String,
    /// Last share
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub last_share: Option<Decimal>,
    /// Last price
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub last_price: Option<Decimal>,
    /// Remark message
    pub remark: String,
}

/// Push event
#[derive(Debug, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum PushEvent {
    /// Order changed
    #[serde(rename = "order_changed_lb")]
    OrderChanged(PushOrderChanged),
}

impl PushEvent {
    pub(crate) fn parse(command_code: u8, data: &[u8]) -> Result<Option<PushEvent>> {
        if command_code == cmd_code::PUSH_NOTIFICATION {
            let notification = Notification::decode(data)?;
            if let Ok(TopicType::Private) = TopicType::from_str(&notification.topic) {
                Ok(Some(serde_json::from_slice::<PushEvent>(
                    &notification.data,
                )?))
            } else {
                Ok(None)
            }
        } else {
            Err(Error::UnknownCommand(command_code))
        }
    }
}
