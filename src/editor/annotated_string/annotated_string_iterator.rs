use std::cmp::min;

use super::{AnnotatedString, AnnotationStringPart};
use crate::prelude::*;

pub struct AnnotatedStringIterator<'a> {
    pub annotated_string: &'a AnnotatedString,
    pub current: ByteIdx,
}

impl<'a> Iterator for AnnotatedStringIterator<'a> {
    type Item = AnnotationStringPart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.annotated_string.string.len() {
            return None;
        }

        // Find the current active annotation
        if let Some(annotation) = self
            .annotated_string
            .annotations
            .iter()
            .filter(|annotation| annotation.start < self.current && annotation.end > self.current)
            .last()
        {
            let end = min(annotation.end, self.annotated_string.string.len());
            let start = self.current;
            self.current = end;
            return Some(AnnotationStringPart {
                string: &self.annotated_string.string[start..end],
                annotation_type: Some(annotation.annotation_type),
            });
        }
        // Find the boundary of the nearest annotation
        let mut end = self.annotated_string.string.len();
        for annotation in &self.annotated_string.annotations {
            if annotation.start > self.current && annotation.start < end {
                end = annotation.start;
            }
        }
        let start = self.current;
        self.current = end;

        Some(AnnotationStringPart {
            string: &self.annotated_string.string[start..end],
            annotation_type: None,
        })
    }
}
