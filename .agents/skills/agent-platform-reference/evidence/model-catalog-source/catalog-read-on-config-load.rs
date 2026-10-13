 2161     let file_contents = std::fs::read_to_string(path)?;
 2162     let catalog = serde_json::from_str::<ModelsResponse>(&file_contents).map_err(|err| {
 2163         std::io::Error::new(
 2164             ErrorKind::InvalidData,
 2165             format!(
 2166                 "failed to parse model_catalog_json path `{}` as JSON: {err}",
 2167                 path.display()
 2168             ),
 2169         )
 2170     })?;
 2171     if catalog.models.is_empty() {
 2172         return Err(std::io::Error::new(
 2173             ErrorKind::InvalidData,
 2174             format!(
 2175                 "model_catalog_json path `{}` must contain at least one model",
 2176                 path.display()
 2177             ),
 2178         ));
 2179     }
 2180     Ok(catalog)
 2181 }
 2182 
 2183 fn load_model_catalog(
 2184     model_catalog_json: Option<AbsolutePathBuf>,
 2185 ) -> std::io::Result<Option<ModelsResponse>> {
 2186     model_catalog_json
 2187         .map(|path| load_catalog_json(&path))
 2188         .transpose()
 2189 }
