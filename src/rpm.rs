use std::io::{BufReader, Read};

use crate::RpmPlugin;
use nu_plugin::PluginCommand;
use nu_protocol::{record, Category, LabeledError, PipelineData, Signature, Type, Value};
use rpm::{chrono, FileEntry, Scriptlet};

fn dep_to_record(dep: &rpm::Dependency, span: nu_protocol::Span) -> nu_protocol::Value {
    let names = dep
        .flags
        .iter_names()
        .map(|(x, _)| x.to_string())
        .collect::<Vec<String>>();

    Value::record(
        record!(
            "name" => Value::string(dep.name.clone(), span),
            "flags" => Value::string(names.join(" | "), span),
            "version" => Value::string(dep.version.clone(), span),
        ),
        span,
    )
}

fn file_entry_to_record(e: &FileEntry, span: nu_protocol::Span) -> nu_protocol::Value {
    let (typ, mode) = match e.mode {
        rpm::FileMode::Dir { permissions } => ("dir", unix_mode::to_string(permissions as u32)),
        rpm::FileMode::Regular { permissions } => {
            ("file", unix_mode::to_string(permissions as u32))
        }
        rpm::FileMode::SymbolicLink { permissions } => {
            ("symlink", unix_mode::to_string(permissions as u32))
        }
        rpm::FileMode::Invalid { raw_mode, reason } => {
            ("invalid", format!("{} {}", raw_mode, reason))
        }
        _ => ("", "".to_owned()),
    };
    Value::record(
        record!(
            "path" => Value::string(e.path.clone().into_os_string().into_string().unwrap_or_default(), span),
            "type" => Value::string(typ, span),
            "mode" => Value::string(mode, span),
            "user" => Value::string(e.ownership.user.clone(), span),
            "group" => Value::string(e.ownership.group.clone(), span),
            "motified_at" => Value::date(chrono::DateTime::from_timestamp(u32::from(e.modified_at) as i64, 0).unwrap_or_default().into(), span),
            "size" => Value::filesize(e.size as i64, span),
            "linkto" => Value::string(e.linkto.clone(), span),
        ),
        span,
    )
}

pub struct FromRpm;

impl FromRpm {
    fn convert_metadata_to_record(
        &self,
        r: impl Read + Send + 'static,
        span: nu_protocol::Span,
        show_files: bool,
    ) -> Result<nu_protocol::Value, rpm::Error> {
        let mut buf_reader = BufReader::new(r);
        let pkg = rpm::Package::parse(&mut buf_reader)?;
        let md = pkg.metadata;

        let mut rec = record!(
            "name" => Value::string(md.get_name().unwrap_or_default(), span),
            "version" => Value::string(md.get_version().unwrap_or_default(), span),
            "release" => Value::string(md.get_release().unwrap_or_default(), span),
            "arch" => Value::string(md.get_arch().unwrap_or_default(), span),
            "size" => Value::filesize(md.get_installed_size().unwrap_or_default() as i64, span),
            "vendor" => Value::string(md.get_vendor().unwrap_or_default(), span),
            "url" => Value::string(md.get_url().unwrap_or_default(), span),
            "vcs" => Value::string(md.get_vcs().unwrap_or_default(), span),
            "license" => Value::string(md.get_license().unwrap_or_default(), span),
            "summary" => Value::string(md.get_summary().unwrap_or_default(), span),
            "description" => Value::string(md.get_description().unwrap_or_default(), span),
            "group" => Value::string(md.get_group().unwrap_or_default(), span),
            "packager" => Value::string(md.get_packager().unwrap_or_default(), span),
            "build_time" => Value::date(chrono::DateTime::from_timestamp(md.get_build_time().unwrap_or_default() as i64, 0).unwrap_or_default().into(), span),
            "build_host" => Value::string(md.get_build_host().unwrap_or_default(), span),
            "cookie" => Value::string(md.get_cookie().unwrap_or_default(), span),
            "source_rpm" => Value::string(md.get_source_rpm().unwrap_or_default(), span),
            "pre_install_script" => Value::string(md.get_pre_install_script().unwrap_or(Scriptlet::new("")).script, span),
            "post_install_script" => Value::string(md.get_post_install_script().unwrap_or(Scriptlet::new("")).script, span),
            "pre_uninstall_script" => Value::string(md.get_pre_uninstall_script().unwrap_or(Scriptlet::new("")).script, span),
            "post_uninstall_script" => Value::string(md.get_post_uninstall_script().unwrap_or(Scriptlet::new("")).script, span),
            "pre_trans_script" => Value::string(md.get_pre_trans_script().unwrap_or(Scriptlet::new("")).script, span),
            "post_trans_script" => Value::string(md.get_post_trans_script().unwrap_or(Scriptlet::new("")).script, span),
            "pre_untrans_script" => Value::string(md.get_pre_untrans_script().unwrap_or(Scriptlet::new("")).script, span),
            "post_untrans_script" => Value::string(md.get_post_untrans_script().unwrap_or(Scriptlet::new("")).script, span),
            "provides" => Value::list(md.get_provides().unwrap_or_default().iter().map(|x| dep_to_record(x, span)).collect::<Vec<_>>(), span),
            "requires" => Value::list(md.get_requires().unwrap_or_default().iter().map(|x| dep_to_record(x, span)).collect::<Vec<_>>(), span),
            "conficts" => Value::list(md.get_conflicts().unwrap_or_default().iter().map(|x| dep_to_record(x, span)).collect::<Vec<_>>(), span),
            "obsoletes" => Value::list(md.get_obsoletes().unwrap_or_default().iter().map(|x| dep_to_record(x, span)).collect::<Vec<_>>(), span),
            "recommends" => Value::list(md.get_recommends().unwrap_or_default().iter().map(|x| dep_to_record(x, span)).collect::<Vec<_>>(), span),
            "suggests" => Value::list(md.get_suggests().unwrap_or_default().iter().map(|x| dep_to_record(x, span)).collect::<Vec<_>>(), span),
        );
        if show_files {
            rec.insert(
                "files",
                Value::list(
                    md.get_file_entries()
                        .unwrap_or_default()
                        .iter()
                        .map(|e| file_entry_to_record(e, span))
                        .collect(),
                    span,
                ),
            );
        }
        Ok(Value::record(rec, span))
    }
}

impl PluginCommand for FromRpm {
    type Plugin = RpmPlugin;

    fn name(&self) -> &str {
        "from rpm"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .switch(
                "files",
                "Convert file lists from rpm package into table",
                Some('f'),
            )
            .allow_variants_without_examples(true)
            .input_output_types(vec![(Type::Binary, Type::Any)])
            .category(Category::Experimental)
            .filter()
    }

    fn description(&self) -> &str {
        "Convert from rpm file into metadata table"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        match input {
            PipelineData::ByteStream(byte_stream, pipeline_metadata) => {
                let span = byte_stream.span();
                let rec = self
                    .convert_metadata_to_record(
                        byte_stream
                            .reader()
                            .ok_or(LabeledError::new("empty input"))?,
                        span,
                        call.has_flag("files")?,
                    )
                    .map_err(|e| LabeledError::new(e.to_string()))?;
                Ok(PipelineData::Value(rec, pipeline_metadata))
            }
            v => {
                return Err(LabeledError::new(format!(
                    "requires binary input, got {}",
                    v.get_type()
                ))
                .with_label("Expected binary from pipeline", call.head))
            }
        }
    }
}
