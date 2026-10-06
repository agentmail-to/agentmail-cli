use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CalendarClient {
    pub http_client: HttpClient,
}

impl CalendarClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Gets the inbox's calendar. Every inbox has one calendar, so this works before any event is
    /// created. Its `etag` (also the `ETag` response header) is the value to send in `If-Match` to
    /// make an update conditional.
    ///
    /// Requires the `calendar_read` permission. Calendar is in private beta: organizations without
    /// access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .get(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesCalendarGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        inbox_id: &InboxesInboxId,
        request: &InboxesCalendarGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Calendar, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/calendar", inbox_id.0),
                None,
                QueryBuilder::new()
                    .serialize("consistency", request.consistency.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates the calendar's default time zone. Existing events keep their own `timezone`; only
    /// events created later without a `timezone` use the new default.
    ///
    /// Requires the `calendar_update` permission. To make the update conditional, send the
    /// calendar's current `etag` in `If-Match`: a stale value returns `412`. Without `If-Match` the
    /// update applies to the calendar as it is.
    ///
    /// Calendar is in private beta: organizations without access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .update(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &UpdateCalendarRequest {
    ///                 timezone: IanaTimezone("America/New_York".to_string()),
    ///             },
    ///             Some(RequestOptions::new().additional_header("If-Match", "\"rv-0\"")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        inbox_id: &InboxesInboxId,
        request: &UpdateCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<Calendar, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/inboxes/{}/calendar", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the events stored on the calendar: one item per one-off or recurring event, plus one
    /// item for each edited date of a recurring event (as that dated event, with
    /// `is_exception: true`). Ordered by most recently updated, and cancelled events are included.
    /// Use it to sync or manage what you created. To see what is on the calendar in a time window,
    /// use Get Agenda.
    ///
    /// The list is always read in the region that serves the request, so it can trail a change made
    /// moments earlier by a few seconds. Requires the `calendar_event_read` permission.
    ///
    /// Calendar is in private beta: organizations without access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .list_events(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &ListEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_events(
        &self,
        inbox_id: &InboxesInboxId,
        request: &ListEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCalendarEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/calendar/events", inbox_id.0),
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a one-off or recurring event on the inbox's calendar. Times are wall-clock values in
    /// `timezone` (the calendar's default time zone if omitted); the response also gives each
    /// boundary as a UTC instant in `start_at` and `end_at`.
    ///
    /// `calendar.event.created` is sent once the event is stored, then `calendar.event.starting`
    /// and `calendar.event.ending` as each date begins and ends. With `send_invites: true` the
    /// inbox also emails an invitation to every attendee.
    ///
    /// Pass `client_id` to make retries safe: repeating the request with the same `client_id` and
    /// body returns the original event with status `200` instead of `201`, for as long as the event
    /// exists.
    ///
    /// With `send_invites: true`, each attendee counts as one send against the organization, pod
    /// and inbox send limits, charged before the event is stored. An over-limit request returns
    /// `429` `rate_limit_exceeded` and creates nothing. A replay of an earlier create is not
    /// charged again.
    ///
    /// The event's `etag` is the value to send in `If-Match` to make a later update or delete
    /// conditional. Requires the `calendar_event_create` permission.
    ///
    /// Calendar is in private beta: organizations without access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .create_event(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &CreateCalendarEventRequest {
    ///                 client_id: Some("intro-acme-2026-10-15".to_string()),
    ///                 title: "Intro call with Acme".to_string(),
    ///                 description: Some("Walk Jane through the onboarding plan.".to_string()),
    ///                 location: Some("https://meet.example.com/acme-intro".to_string()),
    ///                 metadata: Some(serde_json::json!({"crm_deal_id":"D-1042"})),
    ///                 start: WallTime("2026-10-15T14:00:00".to_string()),
    ///                 end: WallTime("2026-10-15T14:30:00".to_string()),
    ///                 timezone: Some(IanaTimezone("America/New_York".to_string())),
    ///                 attendees: Some(vec![Attendee {
    ///                     email: "jane@acme.com".to_string(),
    ///                     name: Some("Jane Doe".to_string()),
    ///                     ..Default::default()
    ///                 }]),
    ///                 send_invites: Some(true),
    ///                 status: None,
    ///                 all_day: None,
    ///                 duration_mode: None,
    ///                 recurrence: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_event(
        &self,
        inbox_id: &InboxesInboxId,
        request: &CreateCalendarEventRequest,
        options: Option<RequestOptions>,
    ) -> Result<CalendarEventMutationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/calendar/events", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists every date on the calendar in a time window, ordered by start time: one-off events,
    /// and recurring events expanded into their individual dates, with cancelled dates left out.
    /// Use it to answer "what is on the calendar".
    ///
    /// The window defaults to now through 90 days from now and can be at most 366 days. Items omit
    /// `description`, `metadata` and `attendees`; get an event by ID for the full object. Dates of
    /// recurring events appear only up to about 90 days from now; use List Event Instances for a
    /// recurring event's later dates. While a recurring event's dates are being regenerated after a
    /// schedule change, which takes a few seconds, the agenda can briefly leave out some of them;
    /// dates that have already started or ended stay as they ran.
    ///
    /// The agenda is read in the region that serves the request, so it can trail a change made
    /// moments earlier by a few seconds. Pass `consistency=primary` to read your own change right
    /// away. Requires the `calendar_event_read` permission.
    ///
    /// Calendar is in private beta: organizations without access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .get_agenda(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &GetAgendaQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_agenda(
        &self,
        inbox_id: &InboxesInboxId,
        request: &GetAgendaQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCalendarEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/calendar/agenda", inbox_id.0),
                None,
                QueryBuilder::new()
                    .serialize("consistency", request.consistency.clone())
                    .serialize("after", request.after.clone())
                    .serialize("before", request.before.clone())
                    .serialize("include_overlapping", request.include_overlapping.clone())
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Gets an event by its UUID, or one date of a recurring event by its dated ID
    /// (`<uuid>_<slot>`). A dated ID returns the date as it currently stands, including any edit
    /// to it, with `kind: instance`.
    ///
    /// The response's `etag` (also the `ETag` header) is the value to send in `If-Match` to make an
    /// update, delete or response to this event or date conditional. Treat it as opaque.
    ///
    /// Reads can trail a change made moments earlier by a few seconds; pass `consistency=primary`
    /// to read the latest state of an event or a date. A date that has already started or ended
    /// reads back as it ran, even if a later change to the series no longer produces it. Requires
    /// the `calendar_event_read` permission.
    ///
    /// Calendar is in private beta: organizations without access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .get_event(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &CalendarEventID("event_id".to_string()),
    ///             &GetEventQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_event(
        &self,
        inbox_id: &InboxesInboxId,
        event_id: &CalendarEventId,
        request: &GetEventQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CalendarEvent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/calendar/events/{}", inbox_id.0, event_id.0),
                None,
                QueryBuilder::new()
                    .serialize("consistency", request.consistency.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes an event, or cancels dates of a recurring event.
    ///
    /// - **One-off or series UUID:** deletes the event and every date of it. The event disappears
    /// from reads immediately and is removed in the background; the response is `202` with a
    /// `deletion_id`. A retry returns the same `deletion_id` while removal runs (send the same
    /// `Idempotency-Key`, or none and the same `send_invites`); once it has finished, the event
    /// no longer exists and a retry returns `404`. No `calendar.event.starting` or
    /// `calendar.event.ending` webhook is sent for the event after the delete is accepted.
    /// - **Dated ID (`<uuid>_<slot>`):** cancels that date (`mode=single`, the default) or that date
    /// and every later date (`mode=future`). Returns `202` with the cancelled date. A `mode=future`
    /// delete from the first date deletes the whole series and returns a `deletion_id` instead.
    /// A date that is already running still gets its `calendar.event.ending`.
    ///
    /// Deleting a one-off or series event sends `calendar.event.deleted`. Cancelling dates sends
    /// `calendar.event.updated` with the cancelled date. With `send_invites=true` the organizer
    /// inbox also emails a cancellation to every attendee. Requires the `calendar_event_delete`
    /// permission. To make the delete conditional, send the current `etag` in `If-Match`.
    ///
    /// Emailing cancellations counts one send per attendee against the organization, pod and inbox
    /// send limits, charged before the delete; an over-limit request returns `429`
    /// `rate_limit_exceeded` and deletes nothing.
    ///
    /// Calendar is in private beta: organizations without access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `send_invites` - When `true`, emails a cancellation (iCalendar `CANCEL`) to every attendee. Only the organizer can send. Defaults to `false`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .delete_event(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &CalendarEventID("event_id".to_string()),
    ///             &DeleteEventQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             Some(
    ///                 RequestOptions::new()
    ///                     .additional_header("If-Match", "\"rv-1\"")
    ///                     .additional_header("Idempotency-Key", "delete-intro-acme"),
    ///             ),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_event(
        &self,
        inbox_id: &InboxesInboxId,
        event_id: &CalendarEventId,
        request: &DeleteEventQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteCalendarEventResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/inboxes/{}/calendar/events/{}", inbox_id.0, event_id.0),
                None,
                QueryBuilder::new()
                    .serialize("mode", request.mode.clone())
                    .serialize("send_invites", request.send_invites.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates an event. Send only the fields to change. To make the update conditional, send the
    /// event's current `etag` in `If-Match`: a stale value returns `412`. Without `If-Match` the
    /// update applies to the event as it is; a change that lands while it runs returns `409`
    /// `race_condition`, so retry. Send `If-Match` when replacing `attendees`, so you don't
    /// overwrite a response that arrived in the meantime.
    ///
    /// - **One-off or series UUID:** changes the event itself. For a series, the change applies to
    /// every date that has not been edited individually.
    /// - **Dated ID (`<uuid>_<slot>`):** changes one date (`mode=single`, the default) or that date
    /// and every later date (`mode=future`). `all_day`, `timezone` and `recurrence` cannot be sent
    /// for a dated ID.
    ///
    /// Once a date has started, its start can no longer change (409 `event_already_started`), but
    /// its end and status can; for a one-off or series UUID, send the unchanged `start` with the new
    /// `end`. Once it has ended, only `title`, `description`, `location`,
    /// `metadata` and `attendees` can change. During the few seconds a date is starting, schedule
    /// changes return 409 `event_starting`; retry shortly.
    ///
    /// Sends `calendar.event.updated`. With `send_invites: true` the organizer inbox also emails the
    /// updated invitation to every attendee. Requires the `calendar_event_update` permission.
    ///
    /// Emailing attendees counts one send per attendee against the organization, pod and inbox
    /// send limits, charged before the change is saved; an over-limit request returns `429`
    /// `rate_limit_exceeded` and changes nothing.
    ///
    /// Calendar is in private beta: organizations without access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .update_event(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &CalendarEventID("event_id".to_string()),
    ///             &UpdateCalendarEventRequest {
    ///                 start: Some(WallTime("2026-10-15T15:00:00".to_string())),
    ///                 end: Some(WallTime("2026-10-15T15:30:00".to_string())),
    ///                 send_invites: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("If-Match", "\"rv-0\"")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_event(
        &self,
        inbox_id: &InboxesInboxId,
        event_id: &CalendarEventId,
        request: &UpdateCalendarEventRequest,
        options: Option<RequestOptions>,
    ) -> Result<CalendarEventMutationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/inboxes/{}/calendar/events/{}", inbox_id.0, event_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("mode", request.mode.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Lists the dates of one recurring event in a time window, in start order, with each date's
    /// edits applied. Dates are computed from the rule, so this works for any window up to 366
    /// days, including dates far in the future. Cancelled dates are left out.
    ///
    /// The window defaults to now through 90 days from now. Items omit `description`, `metadata`
    /// and `attendees`; get a date by its ID for the full object. Like other reads, the list can
    /// trail a change made moments earlier by a few seconds; pass `consistency=primary` to read
    /// your own change right away. Requires the `calendar_event_read` permission.
    ///
    /// Calendar is in private beta: organizations without access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `event_id` - UUID of the recurring event.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .list_event_instances(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &"7c4e9b2a-1f3d-4a8e-b6c5-2e9d0f1a8b47".to_string(),
    ///             &ListEventInstancesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_event_instances(
        &self,
        inbox_id: &InboxesInboxId,
        event_id: &str,
        request: &ListEventInstancesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCalendarEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v0/inboxes/{}/calendar/events/{}/instances",
                    inbox_id.0, event_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("consistency", request.consistency.clone())
                    .serialize("after", request.after.clone())
                    .serialize("before", request.before.clone())
                    .serialize("include_overlapping", request.include_overlapping.clone())
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Accepts, declines or tentatively accepts an invitation the inbox received by email. Pass the
    /// event's UUID to respond for every date, or a dated ID to respond for one date only.
    ///
    /// Only works on `email` events where the inbox is an attendee; anything else returns 409
    /// `calendar_response_invalid`. Updates the inbox's attendee entry and sends
    /// `calendar.event.responded`. With `send_reply` (default `true`) the inbox emails the response
    /// to the organizer.
    ///
    /// Requires the `calendar_event_update` permission. To make the response conditional, send the
    /// current `etag` in `If-Match`.
    ///
    /// A reply email counts as one send against the organization, pod and inbox send limits,
    /// charged before the response is saved; an over-limit request returns `429`
    /// `rate_limit_exceeded` and changes nothing.
    ///
    /// Calendar is in private beta: organizations without access receive a `403`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .inboxes
    ///         .calendar
    ///         .respond_to_event(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &CalendarEventID("event_id".to_string()),
    ///             &RespondCalendarEventRequest {
    ///                 status: RespondStatus::Accepted,
    ///                 comment: Some("See you there.".to_string()),
    ///                 send_reply: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("If-Match", "\"rv-0\"")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn respond_to_event(
        &self,
        inbox_id: &InboxesInboxId,
        event_id: &CalendarEventId,
        request: &RespondCalendarEventRequest,
        options: Option<RequestOptions>,
    ) -> Result<CalendarEventMutationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v0/inboxes/{}/calendar/events/{}/respond",
                    inbox_id.0, event_id.0
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
