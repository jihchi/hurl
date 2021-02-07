use crate::runner;
use crate::runner::RunnerError;
use hurl_core::ast::SourceInfo;
use hurl_core::error::Error;

///
/// Textual Output for runner errors
///
///

impl Error for runner::Error {
    fn source_info(&self) -> SourceInfo {
        self.clone().source_info
    }

    fn description(&self) -> String {
        match &self.inner {
            RunnerError::InvalidURL(..) => "Invalid url".to_string(),
            RunnerError::TemplateVariableNotDefined { .. } => "Undefined Variable".to_string(),
            RunnerError::VariableNotDefined { .. } => "Undefined Variable".to_string(),
            RunnerError::HttpConnection { .. } => "Http Connection".to_string(),
            RunnerError::CouldNotResolveProxyName => "Http Connection".to_string(),
            RunnerError::CouldNotResolveHost => "Http Connection".to_string(),
            RunnerError::FailToConnect => "Http Connection".to_string(),
            RunnerError::Timeout => "Http Connection".to_string(),
            RunnerError::TooManyRedirect => "Http Connection".to_string(),
            RunnerError::CouldNotParseResponse => "Http Connection".to_string(),
            RunnerError::SSLCertificate { .. } => "SSL Certificate".to_string(),
            RunnerError::PredicateValue { .. } => "Assert - Predicate Value Failed".to_string(),
            RunnerError::InvalidRegex {} => "Invalid regex".to_string(),
            RunnerError::FileReadAccess { .. } => "File ReadAccess".to_string(),
            RunnerError::QueryInvalidXml { .. } => "Invalid XML".to_string(),
            RunnerError::QueryInvalidXpathEval {} => "Invalid xpath expression".to_string(),
            RunnerError::QueryHeaderNotFound {} => "Header not Found".to_string(),
            RunnerError::QueryCookieNotFound {} => "Cookie not Found".to_string(),
            RunnerError::AssertHeaderValueError { .. } => "Assert Header Value".to_string(),
            RunnerError::AssertBodyValueError { .. } => "Assert Body Value".to_string(),
            RunnerError::AssertVersion { .. } => "Assert Http Version".to_string(),
            RunnerError::AssertStatus { .. } => "Assert Status".to_string(),
            RunnerError::QueryInvalidJson { .. } => "Invalid Json".to_string(),
            RunnerError::QueryInvalidJsonpathExpression { .. } => "Invalid jsonpath".to_string(),
            RunnerError::PredicateType { .. } => "Assert - Inconsistent predicate type".to_string(),
            RunnerError::SubqueryInvalidInput { .. } => "Subquery error".to_string(),
            RunnerError::InvalidDecoding { .. } => "Invalid Decoding".to_string(),
            RunnerError::InvalidCharset { .. } => "Invalid Charset".to_string(),
            RunnerError::AssertFailure { .. } => "Assert Failure".to_string(),
            RunnerError::UnrenderableVariable { .. } => "Unrenderable Variable".to_string(),
            RunnerError::NoQueryResult { .. } => "No query result".to_string(),
            RunnerError::UnsupportedContentEncoding(..) => "Decompression Error".to_string(),
            RunnerError::CouldNotUncompressResponse(..) => "Decompression Error".to_string(),
            RunnerError::InvalidJson { .. } => "Invalid Json".to_string(),
        }
    }

    fn fixme(&self) -> String {
        match &self.inner {
            RunnerError::InvalidURL(url) => format!("Invalid url <{}>", url),
            RunnerError::TemplateVariableNotDefined { name } => {
                format!("You must set the variable {}", name)
            }
            RunnerError::HttpConnection { url, message } => {
                format!("can not connect to {} ({})", url, message)
            }
            RunnerError::CouldNotResolveProxyName => "Could not resolve proxy name".to_string(),
            RunnerError::CouldNotResolveHost => "Could not resolve host".to_string(),
            RunnerError::FailToConnect => "Fail to connect".to_string(),
            RunnerError::Timeout => "Timeout has been reached".to_string(),
            RunnerError::TooManyRedirect => "Too many redirect".to_string(),
            RunnerError::CouldNotParseResponse => "Could not parse response".to_string(),
            RunnerError::SSLCertificate(description) => description.clone(),
            RunnerError::AssertVersion { actual, .. } => format!("actual value is <{}>", actual),
            RunnerError::AssertStatus { actual, .. } => format!("actual value is <{}>", actual),
            RunnerError::PredicateValue(value) => {
                format!("actual value is <{}>", value.to_string())
            }
            RunnerError::InvalidRegex {} => "Regex expression is not valid".to_string(),
            RunnerError::FileReadAccess { value } => format!("File {} can not be read", value),
            RunnerError::QueryInvalidXml { .. } => {
                "The Http response is not a valid XML".to_string()
            }
            RunnerError::QueryHeaderNotFound {} => {
                "This header has not been found in the response".to_string()
            }
            RunnerError::QueryCookieNotFound {} => {
                "This cookie has not been found in the response".to_string()
            }
            RunnerError::QueryInvalidXpathEval {} => {
                "The xpath expression is not valid".to_string()
            }
            RunnerError::AssertHeaderValueError { actual } => {
                format!("actual value is <{}>", actual)
            }
            RunnerError::AssertBodyValueError { actual, .. } => {
                format!("actual value is <{}>", actual)
            }
            RunnerError::QueryInvalidJson { .. } => {
                "The http response is not a valid json".to_string()
            }
            RunnerError::QueryInvalidJsonpathExpression { value } => {
                format!("the jsonpath expression '{}' is not valid", value)
            }
            RunnerError::PredicateType { .. } => {
                "predicate type inconsistent with value return by query".to_string()
            }
            RunnerError::SubqueryInvalidInput => {
                "Type from query result and subquery do not match".to_string()
            }
            RunnerError::InvalidDecoding { charset } => {
                format!("The body can not be decoded with charset '{}'", charset)
            }
            RunnerError::InvalidCharset { charset } => {
                format!("The charset '{}' is not valid", charset)
            }
            RunnerError::AssertFailure {
                actual,
                expected,
                type_mismatch,
                ..
            } => {
                let additional = if *type_mismatch {
                    "\n>>> types between actual and expected are not consistent"
                } else {
                    ""
                };
                format!("actual:   {}\nexpected: {}{}", actual, expected, additional)
            }
            RunnerError::VariableNotDefined { name } => {
                format!("You must set the variable {}", name)
            }
            RunnerError::UnrenderableVariable { value } => {
                format!("value {} can not be rendered", value)
            }
            RunnerError::NoQueryResult { .. } => "The query didn't return any result".to_string(),
            RunnerError::UnsupportedContentEncoding(algorithm) => {
                format!("Compression {} is not supported", algorithm)
            }
            RunnerError::CouldNotUncompressResponse(algorithm) => {
                format!("Could not uncompress response with {}", algorithm)
            }
            RunnerError::InvalidJson { value } => {
                format!("actual value is <{}>", value)
            }
        }
    }
}
