use std::collections::HashMap;
use toml::Value;

pub fn parse_cargo_toml() -> Result<HashMap<String, Value>, toml::de::Error> {
  // 从包含的字符串中读取 Cargo.toml 内容
  let file_content = include_str!("../../Cargo.toml");

  // 解析 TOML 数据
  let toml_data = file_content.parse::<Value>()?;

  // 检查是否存在 [package] 表
  if let Some(package) = toml_data.get("package") {
      // 如果存在 [package] 表，则将其内容转换为 HashMap<String, Value>
      if let Some(package_table) = package.as_table() {
          let cargo_toml_map: HashMap<String, Value> = package_table.clone().into_iter().collect();
          Ok(cargo_toml_map)
      } else {
          Err(<toml::de::Error as serde::de::Error>::custom("Invalid TOML data under [package]"))
      }
  } else {
      Err(<toml::de::Error as serde::de::Error>::custom("Missing [package] section in Cargo.toml"))
  }
}