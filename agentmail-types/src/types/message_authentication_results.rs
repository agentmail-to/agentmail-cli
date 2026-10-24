pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Per-mechanism SPF, DKIM, and DMARC verdicts for a received message, each with the domain it
/// is about, kept only where they speak for the sender's From identity. A verdict that could
/// not be computed is absent, so the field can be `{}`. The field is absent on sent messages
/// and on messages received before 2026-09-09; the `*_domain` and `arc_sealer` keys appear on
/// messages received after they were introduced.
/// 
/// The verdicts are the ones Amazon SES computed when the message reached AgentMail, with one
/// exception. Forwarded mail fails DMARC at the receiving hop by construction. When AgentMail
/// delivers a forwarded message as authenticated because a trusted forwarder (such as Google or
/// Microsoft) sealed a valid ARC chain vouching for the original, the field reports the
/// verdicts that forwarder recorded for the original message, and `arc_sealer` names it.
/// 
/// - `spf` is about the envelope MAIL FROM (Return-Path) domain, named in `spf_domain`. It is
/// present only when that domain is aligned with the sender's domain (the From mailbox, or
/// the Sender header when From spans domains), meaning the two share an organizational
/// domain: `mail.example.com` is aligned with `example.com`. An unaligned SPF result is
/// withheld whatever its value.
/// - `dkim` is the DKIM verdict, and `dkim_domain` the signing domain (the `d=` tag). On
/// directly received mail the domain is named only when every DKIM signature on the message
/// is from the same domain; a message signed by more than one domain (a sender and its email
/// provider, for instance) keeps the verdict and omits the domain. `dkim_domain` is the
/// signing domain as written, so compare it with the From domain before relying on it.
/// - `dmarc` is the DMARC verdict for the From domain, named in `dmarc_domain`.
/// - `dkim` and `dmarc` are withheld when the From list spans more than one domain or could not
/// be fully parsed.
/// - On forwarded mail each sealed verdict is reported only when the domain the forwarder
/// recorded it for matches the message's current From domain. If the forwarder rewrote From,
/// or vouched for nothing, the field is `{}`.
/// 
/// The `unauthenticated` label, not this field, is AgentMail's acceptance decision, and it uses
/// a looser rule, so the two can disagree: a single-From message whose only pass is an
/// unaligned envelope SPF is delivered without the label while this field shows no pass. Use
/// the label to tell whether AgentMail accepted the message as legitimate, and this field to
/// see which domain each mechanism authenticated. Do not reject mail on `dmarc: fail` alone.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessageAuthenticationResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spf: Option<MessageAuthenticationVerdict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim: Option<MessageAuthenticationVerdict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmarc: Option<MessageAuthenticationVerdict>,
    /// Envelope MAIL FROM domain the `spf` verdict is about (`smtp.mailfrom`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spf_domain: Option<MessageAuthenticationDomain>,
    /// Signing domain the `dkim` verdict is about (`header.d`). Present only when `dkim` is
    /// `pass` or `fail`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_domain: Option<MessageAuthenticationDomain>,
    /// From domain the `dmarc` verdict is about (`header.from`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmarc_domain: Option<MessageAuthenticationDomain>,
    /// Domain of the trusted forwarder whose ARC seal the verdicts were taken from, such as
    /// `google.com`. Present only on forwarded mail, and only beside at least one verdict.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_sealer: Option<MessageAuthenticationDomain>,
}

impl MessageAuthenticationResults {
    pub fn builder() -> MessageAuthenticationResultsBuilder {
        <MessageAuthenticationResultsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageAuthenticationResultsBuilder {
    spf: Option<MessageAuthenticationVerdict>,
    dkim: Option<MessageAuthenticationVerdict>,
    dmarc: Option<MessageAuthenticationVerdict>,
    spf_domain: Option<MessageAuthenticationDomain>,
    dkim_domain: Option<MessageAuthenticationDomain>,
    dmarc_domain: Option<MessageAuthenticationDomain>,
    arc_sealer: Option<MessageAuthenticationDomain>,
}

impl MessageAuthenticationResultsBuilder {
    pub fn spf(mut self, value: MessageAuthenticationVerdict) -> Self {
        self.spf = Some(value);
        self
    }

    pub fn dkim(mut self, value: MessageAuthenticationVerdict) -> Self {
        self.dkim = Some(value);
        self
    }

    pub fn dmarc(mut self, value: MessageAuthenticationVerdict) -> Self {
        self.dmarc = Some(value);
        self
    }

    pub fn spf_domain(mut self, value: MessageAuthenticationDomain) -> Self {
        self.spf_domain = Some(value);
        self
    }

    pub fn dkim_domain(mut self, value: MessageAuthenticationDomain) -> Self {
        self.dkim_domain = Some(value);
        self
    }

    pub fn dmarc_domain(mut self, value: MessageAuthenticationDomain) -> Self {
        self.dmarc_domain = Some(value);
        self
    }

    pub fn arc_sealer(mut self, value: MessageAuthenticationDomain) -> Self {
        self.arc_sealer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageAuthenticationResults`].
    pub fn build(self) -> Result<MessageAuthenticationResults, BuildError> {
        Ok(MessageAuthenticationResults {
            spf: self.spf,
            dkim: self.dkim,
            dmarc: self.dmarc,
            spf_domain: self.spf_domain,
            dkim_domain: self.dkim_domain,
            dmarc_domain: self.dmarc_domain,
            arc_sealer: self.arc_sealer,
        })
    }
}
