use super::AnnotationType;
#[derive(Debug)]
pub struct AnnotationStringPart<'a> {
    pub string: &'a str,
    pub annotation_type: Option<AnnotationType>,
}
