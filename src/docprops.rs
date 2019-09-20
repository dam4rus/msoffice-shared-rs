use crate::xml::XmlNode;
use std::io::{Read, Seek};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AppInfo {
    pub app_name: Option<String>,
    pub app_version: Option<String>,
}

impl AppInfo {
    pub fn from_zip<R>(zipper: &mut zip::ZipArchive<R>) -> Result<Self, Box<dyn (::std::error::Error)>>
    where
        R: Read + Seek,
    {
        let mut app_xml_file = zipper.by_name("docProps/app.xml")?;

        let mut xml_string = String::new();
        app_xml_file.read_to_string(&mut xml_string)?;
        let root = XmlNode::from_str(&xml_string)?;

        root.child_nodes.iter().try_fold(Default::default(), |mut instance: Self, child_node| {
            match child_node.local_name() {
                "Application" => instance.app_name = child_node.text.as_ref().cloned(),
                "AppVersion" => instance.app_version = child_node.text.as_ref().cloned(),
                _ => (),
            }

            Ok(instance)
        })
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Core {
    pub title: Option<String>,
    pub creator: Option<String>,
    pub last_modified_by: Option<String>,
    pub revision: Option<i32>,
    pub created_time: Option<String>,  // TODO: maybe store as some DateTime struct?
    pub modified_time: Option<String>, // TODO: maybe store as some DateTime struct?
}

impl Core {
    pub fn from_zip<R>(zipper: &mut zip::ZipArchive<R>) -> Result<Self, Box<dyn (::std::error::Error)>>
    where
        R: Read + Seek,
    {
        let mut core_xml_file = zipper.by_name("docProps/core.xml")?;
        let mut xml_string = String::new();
        core_xml_file.read_to_string(&mut xml_string)?;
        let root = XmlNode::from_str(&xml_string)?;

        root.child_nodes.iter().try_fold(Default::default(), |mut instance: Self, child_node|{
            match child_node.local_name() {
                "title" => instance.title = child_node.text.as_ref().cloned(),
                "creator" => instance.creator = child_node.text.as_ref().cloned(),
                "lastModifiedBy" => instance.last_modified_by = child_node.text.as_ref().cloned(),
                "revision" => instance.revision = child_node.text.as_ref().map(|s| s.parse()).transpose()?,
                "created" => instance.created_time = child_node.text.as_ref().cloned(),
                "modified" => instance.modified_time = child_node.text.as_ref().cloned(),
                _ => (),
            }

            Ok(instance)
        })
    }
}
