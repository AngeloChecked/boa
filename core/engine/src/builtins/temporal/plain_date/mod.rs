//! Boa's implementation of the ECMAScript `Temporal.PlainDate` builtin object.
#![allow(dead_code, unused_variables)]

// TODO (nekevss): DOCS DOCS AND MORE DOCS

use crate::{
    builtins::{
        options::{get_option, get_options_object},
        BuiltInBuilder, BuiltInConstructor, BuiltInObject, IntrinsicObject,
    },
    context::intrinsics::{Intrinsics, StandardConstructor, StandardConstructors},
    js_string,
    object::internal_methods::get_prototype_from_constructor,
    property::Attribute,
    realm::Realm,
    string::{common::StaticJsStrings, utf16},
    Context, JsArgs, JsData, JsNativeError, JsObject, JsResult, JsString, JsSymbol, JsValue,
};
use boa_gc::{Finalize, Trace};
use boa_profiler::Profiler;
use boa_temporal::{
    components::{
        calendar::{CalendarSlot, GetCalendarSlot},
        Date as InnerDate, DateTime,
    },
    iso::IsoDateSlots,
    options::ArithmeticOverflow,
};

use super::{calendar, create_temporal_calendar, PlainDateTime, ZonedDateTime};

/// The `Temporal.PlainDate` object.
#[derive(Debug, Clone, Trace, Finalize, JsData)]
#[boa_gc(unsafe_empty_trace)] // TODO: Remove this!!! `InnerDate` could contain `Trace` types.
pub struct PlainDate {
    pub(crate) inner: InnerDate<JsObject>,
}

impl PlainDate {
    pub(crate) fn new(inner: InnerDate<JsObject>) -> Self {
        Self { inner }
    }
}

impl IsoDateSlots for JsObject<PlainDate> {
    fn iso_date(&self) -> boa_temporal::iso::IsoDate {
        self.borrow().data().inner.iso()
    }
}

impl GetCalendarSlot<JsObject> for JsObject<PlainDate> {
    fn get_calendar(&self) -> CalendarSlot<JsObject> {
        self.borrow().data().inner.get_calendar()
    }
}

impl BuiltInObject for PlainDate {
    const NAME: JsString = StaticJsStrings::PLAIN_DATE;
}

impl IntrinsicObject for PlainDate {
    fn init(realm: &Realm) {
        let _timer = Profiler::global().start_event(std::any::type_name::<Self>(), "init");

        let get_calendar_id = BuiltInBuilder::callable(realm, Self::get_calendar_id)
            .name(js_string!("get calendarId"))
            .build();

        let get_year = BuiltInBuilder::callable(realm, Self::get_year)
            .name(js_string!("get year"))
            .build();

        let get_month = BuiltInBuilder::callable(realm, Self::get_month)
            .name(js_string!("get month"))
            .build();

        let get_month_code = BuiltInBuilder::callable(realm, Self::get_month_code)
            .name(js_string!("get monthCode"))
            .build();

        let get_day = BuiltInBuilder::callable(realm, Self::get_day)
            .name(js_string!("get day"))
            .build();

        let get_day_of_week = BuiltInBuilder::callable(realm, Self::get_day_of_week)
            .name(js_string!("get dayOfWeek"))
            .build();

        let get_day_of_year = BuiltInBuilder::callable(realm, Self::get_day_of_year)
            .name(js_string!("get dayOfYear"))
            .build();

        let get_week_of_year = BuiltInBuilder::callable(realm, Self::get_week_of_year)
            .name(js_string!("get weekOfYear"))
            .build();

        let get_year_of_week = BuiltInBuilder::callable(realm, Self::get_year_of_week)
            .name(js_string!("get yearOfWeek"))
            .build();

        let get_days_in_week = BuiltInBuilder::callable(realm, Self::get_days_in_week)
            .name(js_string!("get daysInWeek"))
            .build();

        let get_days_in_month = BuiltInBuilder::callable(realm, Self::get_days_in_month)
            .name(js_string!("get daysInMonth"))
            .build();

        let get_days_in_year = BuiltInBuilder::callable(realm, Self::get_days_in_year)
            .name(js_string!("get daysInYear"))
            .build();

        let get_months_in_year = BuiltInBuilder::callable(realm, Self::get_months_in_year)
            .name(js_string!("get monthsInYear"))
            .build();

        let get_in_leap_year = BuiltInBuilder::callable(realm, Self::get_in_leap_year)
            .name(js_string!("get inLeapYear"))
            .build();

        BuiltInBuilder::from_standard_constructor::<Self>(realm)
            .property(
                JsSymbol::to_string_tag(),
                Self::NAME,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("calendarId"),
                Some(get_calendar_id),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("year"),
                Some(get_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("month"),
                Some(get_month),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("monthCode"),
                Some(get_month_code),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(utf16!("day"), Some(get_day), None, Attribute::CONFIGURABLE)
            .accessor(
                utf16!("dayOfWeek"),
                Some(get_day_of_week),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("dayOfYear"),
                Some(get_day_of_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("weekOfYear"),
                Some(get_week_of_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("yearOfWeek"),
                Some(get_year_of_week),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("daysInWeek"),
                Some(get_days_in_week),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("daysInMonth"),
                Some(get_days_in_month),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("daysInYear"),
                Some(get_days_in_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("monthsInYear"),
                Some(get_months_in_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                utf16!("inLeapYear"),
                Some(get_in_leap_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .method(Self::to_plain_year_month, js_string!("toPlainYearMonth"), 0)
            .method(Self::to_plain_month_day, js_string!("toPlainMonthDay"), 0)
            .method(Self::get_iso_fields, js_string!("getISOFields"), 0)
            .method(Self::get_calendar, js_string!("getCalendar"), 0)
            .method(Self::add, js_string!("add"), 2)
            .method(Self::subtract, js_string!("subtract"), 2)
            .method(Self::with, js_string!("with"), 2)
            .method(Self::with_calendar, js_string!("withCalendar"), 1)
            .method(Self::until, js_string!("until"), 2)
            .method(Self::since, js_string!("since"), 2)
            .method(Self::equals, js_string!("equals"), 1)
            .build();
    }

    fn get(intrinsics: &Intrinsics) -> JsObject {
        Self::STANDARD_CONSTRUCTOR(intrinsics.constructors()).constructor()
    }
}

impl BuiltInConstructor for PlainDate {
    const LENGTH: usize = 0;

    const STANDARD_CONSTRUCTOR: fn(&StandardConstructors) -> &StandardConstructor =
        StandardConstructors::plain_date;

    fn constructor(
        new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        if new_target.is_undefined() {
            return Err(JsNativeError::typ()
                .with_message("NewTarget cannot be undefined.")
                .into());
        };

        let iso_year = super::to_integer_with_truncation(args.get_or_undefined(0), context)?;
        let iso_month = super::to_integer_with_truncation(args.get_or_undefined(1), context)?;
        let iso_day = super::to_integer_with_truncation(args.get_or_undefined(2), context)?;
        let calendar_slot =
            calendar::to_temporal_calendar_slot_value(args.get_or_undefined(3), context)?;

        let date = InnerDate::new(
            iso_year,
            iso_month,
            iso_day,
            calendar_slot,
            ArithmeticOverflow::Reject,
        )?;

        Ok(create_temporal_date(date, Some(new_target), context)?.into())
    }
}

// ==== `PlainDate` getter methods ====

impl PlainDate {
    /// 3.3.3 get `Temporal.PlainDate.prototype.calendarId`
    fn get_calendar_id(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let date = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a PlainDate object.")
            })?;

        Ok(JsString::from(date.inner.calendar().identifier(context)?).into())
    }

    /// 3.3.4 get `Temporal.PlainDate.prototype.year`
    fn get_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_year(&date, context)?.into())
    }

    /// 3.3.5 get `Temporal.PlainDate.prototype.month`
    fn get_month(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_month(&date, context)?.into())
    }

    /// 3.3.6 get Temporal.PlainDate.prototype.monthCode
    fn get_month_code(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(
            JsString::from(InnerDate::<JsObject>::contextual_month_code(&date, context)?.as_str())
                .into(),
        )
    }

    /// 3.3.7 get `Temporal.PlainDate.prototype.day`
    fn get_day(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_day(&date, context)?.into())
    }

    /// 3.3.8 get `Temporal.PlainDate.prototype.dayOfWeek`
    fn get_day_of_week(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_day_of_week(&date, context)?.into())
    }

    /// 3.3.9 get `Temporal.PlainDate.prototype.dayOfYear`
    fn get_day_of_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_day_of_year(&date, context)?.into())
    }

    /// 3.3.10 get `Temporal.PlainDate.prototype.weekOfYear`
    fn get_week_of_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_week_of_year(&date, context)?.into())
    }

    /// 3.3.11 get `Temporal.PlainDate.prototype.yearOfWeek`
    fn get_year_of_week(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_year_of_week(&date, context)?.into())
    }

    /// 3.3.12 get `Temporal.PlainDate.prototype.daysInWeek`
    fn get_days_in_week(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_days_in_week(&date, context)?.into())
    }

    /// 3.3.13 get `Temporal.PlainDate.prototype.daysInMonth`
    fn get_days_in_month(
        this: &JsValue,
        _: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_days_in_month(&date, context)?.into())
    }

    /// 3.3.14 get `Temporal.PlainDate.prototype.daysInYear`
    fn get_days_in_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_days_in_year(&date, context)?.into())
    }

    /// 3.3.15 get `Temporal.PlainDate.prototype.monthsInYear`
    fn get_months_in_year(
        this: &JsValue,
        _: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_months_in_year(&date, context)?.into())
    }

    /// 3.3.16 get `Temporal.PlainDate.prototype.inLeapYear`
    fn get_in_leap_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let obj = this
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("this must be an object."))?;

        let Ok(date) = obj.clone().downcast::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("the this object must be a PlainDate object.")
                .into());
        };

        Ok(InnerDate::<JsObject>::contextual_in_leap_year(&date, context)?.into())
    }
}

// ==== `PlainDate.prototype` method implementation ====

impl PlainDate {
    fn to_plain_year_month(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }

    fn to_plain_month_day(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }

    fn get_iso_fields(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }

    /// 3.3.20 `Temporal.PlainDate.prototype.getCalendar ( )`
    fn get_calendar(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let date = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a PlainDate object.")
            })?;

        create_temporal_calendar(date.inner.calendar().clone(), None, context)
    }

    fn add(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }

    fn subtract(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }

    fn with(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }

    fn with_calendar(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }

    fn until(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }

    fn since(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }

    fn equals(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::error()
            .with_message("not yet implemented.")
            .into())
    }
}

// -- `PlainDate` Abstract Operations --

impl PlainDate {
    /// Utitily function for translating a `Temporal.PlainDate` into a `JsObject`.
    pub(crate) fn as_object(&self, context: &mut Context) -> JsResult<JsObject> {
        create_temporal_date(self.inner.clone(), None, context)
    }
}

// 3.5.2 `CreateIsoDateRecord`
// Implemented on `IsoDateRecord`

/// 3.5.3 `CreateTemporalDate ( isoYear, isoMonth, isoDay, calendar [ , newTarget ] )`
pub(crate) fn create_temporal_date(
    inner: InnerDate<JsObject>,
    new_target: Option<&JsValue>,
    context: &mut Context,
) -> JsResult<JsObject> {
    // NOTE (nekevss): The below should never trigger as `IsValidISODate` is enforced by Date.
    // 1. If IsValidISODate(isoYear, isoMonth, isoDay) is false, throw a RangeError exception.

    // 2. If ISODateTimeWithinLimits(isoYear, isoMonth, isoDay, 12, 0, 0, 0, 0, 0) is false, throw a RangeError exception.
    if !DateTime::<JsObject>::validate(&inner) {
        return Err(JsNativeError::range()
            .with_message("Date is not within ISO date time limits.")
            .into());
    }

    // 3. If newTarget is not present, set newTarget to %Temporal.PlainDate%.
    let new_target = if let Some(new_target) = new_target {
        new_target.clone()
    } else {
        context
            .realm()
            .intrinsics()
            .constructors()
            .plain_date()
            .constructor()
            .into()
    };

    // 4. Let object be ? OrdinaryCreateFromConstructor(newTarget, "%Temporal.PlainDate.prototype%", « [[InitializedTemporalDate]], [[ISOYear]], [[ISOMonth]], [[ISODay]], [[Calendar]] »).
    let prototype =
        get_prototype_from_constructor(&new_target, StandardConstructors::plain_date, context)?;

    // 5. Set object.[[ISOYear]] to isoYear.
    // 6. Set object.[[ISOMonth]] to isoMonth.
    // 7. Set object.[[ISODay]] to isoDay.
    // 8. Set object.[[Calendar]] to calendar.
    let obj = JsObject::from_proto_and_data(prototype, PlainDate::new(inner));

    // 9. Return object.
    Ok(obj)
}

/// 3.5.4 `ToTemporalDate ( item [ , options ] )`
///
/// Converts an ambiguous `JsValue` into a `PlainDate`
pub(crate) fn to_temporal_date(
    item: &JsValue,
    options: Option<JsValue>,
    context: &mut Context,
) -> JsResult<PlainDate> {
    // 1. If options is not present, set options to undefined.
    let options = options.unwrap_or(JsValue::undefined());

    // 2. Assert: Type(options) is Object or Undefined.
    // 3. If options is not undefined, set options to ? SnapshotOwnProperties(? GetOptionsObject(options), null).
    let options_obj = get_options_object(&options)?;

    // 4. If Type(item) is Object, then
    if let Some(object) = item.as_object() {
        // a. If item has an [[InitializedTemporalDate]] internal slot, then
        if let Some(date) = object.downcast_ref::<PlainDate>() {
            return Ok(PlainDate::new(date.inner.clone()));
        // b. If item has an [[InitializedTemporalZonedDateTime]] internal slot, then
        } else if let Some(data) = object.downcast_ref::<ZonedDateTime>() {
            return Err(JsNativeError::range()
                .with_message("ZonedDateTime not yet implemented.")
                .into());
            // i. Perform ? ToTemporalOverflow(options).
            // ii. Let instant be ! CreateTemporalInstant(item.[[Nanoseconds]]).
            // iii. Let plainDateTime be ? GetPlainDateTimeFor(item.[[TimeZone]], instant, item.[[Calendar]]).
            // iv. Return ! CreateTemporalDate(plainDateTime.[[ISOYear]], plainDateTime.[[ISOMonth]], plainDateTime.[[ISODay]], plainDateTime.[[Calendar]]).

            // c. If item has an [[InitializedTemporalDateTime]] internal slot, then
        } else if let Some(date_time) = object.downcast_ref::<PlainDateTime>() {
            // i. Perform ? ToTemporalOverflow(options).
            let _o = get_option(&options_obj, utf16!("overflow"), context)?
                .unwrap_or(ArithmeticOverflow::Constrain);

            let date = InnerDate::from_datetime(date_time.inner());

            // ii. Return ! CreateTemporalDate(item.[[ISOYear]], item.[[ISOMonth]], item.[[ISODay]], item.[[Calendar]]).
            return Ok(PlainDate::new(date));
        }

        // d. Let calendar be ? GetTemporalCalendarSlotValueWithISODefault(item).
        // e. Let fieldNames be ? CalendarFields(calendar, « "day", "month", "monthCode", "year" »).
        // f. Let fields be ? PrepareTemporalFields(item, fieldNames, «»).
        // g. Return ? CalendarDateFromFields(calendar, fields, options).
        return Err(JsNativeError::error()
            .with_message("CalendarDateFields not yet implemented.")
            .into());
    }

    // 5. If item is not a String, throw a TypeError exception.
    let JsValue::String(date_like_string) = item else {
        return Err(JsNativeError::typ()
            .with_message("ToTemporalDate item must be an object or string.")
            .into());
    };

    // 6. Let result be ? ParseTemporalDateString(item).
    // 7. Assert: IsValidISODate(result.[[Year]], result.[[Month]], result.[[Day]]) is true.
    // 8. Let calendar be result.[[Calendar]].
    // 9. If calendar is undefined, set calendar to "iso8601".
    // 10. If IsBuiltinCalendar(calendar) is false, throw a RangeError exception.
    // 11. Set calendar to the ASCII-lowercase of calendar.
    // 12. Perform ? ToTemporalOverflow(options).
    // 13. Return ? CreateTemporalDate(result.[[Year]], result.[[Month]], result.[[Day]], calendar).
    let result = date_like_string
        .to_std_string_escaped()
        .parse::<InnerDate<JsObject>>()
        .map_err(|err| JsNativeError::range().with_message(err.to_string()))?;

    Ok(PlainDate::new(result))
}
