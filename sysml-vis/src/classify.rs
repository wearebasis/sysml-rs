use sysml_core::{ElementKind, RelationshipKind};

pub(crate) fn is_membership_kind(kind: &ElementKind) -> bool {
    *kind == ElementKind::Membership || kind.is_subtype_of(ElementKind::Membership)
}

pub(crate) fn is_requirement_relationship(kind: &RelationshipKind) -> bool {
    matches!(
        kind,
        RelationshipKind::Satisfy
            | RelationshipKind::Verify
            | RelationshipKind::Derive
            | RelationshipKind::Trace
    )
}

pub(crate) fn is_requirement_kind(kind: &ElementKind) -> bool {
    is_kind_or_subtype(kind, ElementKind::RequirementDefinition)
        || is_kind_or_subtype(kind, ElementKind::RequirementUsage)
        || matches!(
            kind,
            ElementKind::VerificationCaseDefinition | ElementKind::VerificationCaseUsage
        )
}

pub(crate) fn is_state_kind(kind: &ElementKind) -> bool {
    matches!(kind, ElementKind::StateDefinition | ElementKind::StateUsage)
}

pub(crate) fn is_interconnection_kind(kind: &ElementKind) -> bool {
    is_kind_or_subtype(kind, ElementKind::PartDefinition)
        || is_kind_or_subtype(kind, ElementKind::PartUsage)
        || is_kind_or_subtype(kind, ElementKind::PortDefinition)
        || is_kind_or_subtype(kind, ElementKind::PortUsage)
        || is_kind_or_subtype(kind, ElementKind::ConnectionDefinition)
        || is_kind_or_subtype(kind, ElementKind::ConnectionUsage)
        || is_kind_or_subtype(kind, ElementKind::FlowDefinition)
        || is_kind_or_subtype(kind, ElementKind::FlowUsage)
        || is_kind_or_subtype(kind, ElementKind::InterfaceDefinition)
        || is_kind_or_subtype(kind, ElementKind::InterfaceUsage)
        || is_kind_or_subtype(kind, ElementKind::ItemDefinition)
        || is_kind_or_subtype(kind, ElementKind::ItemUsage)
        || is_kind_or_subtype(kind, ElementKind::Connector)
}

pub(crate) fn is_part_kind(kind: &ElementKind) -> bool {
    is_kind_or_subtype(kind, ElementKind::PartDefinition)
        || is_kind_or_subtype(kind, ElementKind::PartUsage)
}

pub(crate) fn is_port_kind(kind: &ElementKind) -> bool {
    is_kind_or_subtype(kind, ElementKind::PortDefinition)
        || is_kind_or_subtype(kind, ElementKind::PortUsage)
}

fn is_kind_or_subtype(kind: &ElementKind, base: ElementKind) -> bool {
    *kind == base || kind.is_subtype_of(base)
}
