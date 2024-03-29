use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AttachRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: AttachRequestArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'attach' request. Additional attributes are implementation specific."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct AttachRequestArguments {
    #[doc = " Optional data from the previous, restarted session."]
    #[doc = " The data is sent as the 'restart' attribute of the 'terminated' event."]
    #[doc = " The client should leave the data intact."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "__restart")]
    pub _restart: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AttachResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Information about a Breakpoint created in setBreakpoints, setFunctionBreakpoints, "]
#[doc = " setInstructionBreakpoints, or setDataBreakpoints."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Breakpoint {
    #[doc = " An optional start column of the actual range covered by the breakpoint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[doc = " An optional end column of the actual range covered by the breakpoint."]
    #[doc = " If no end line is given, then the end column is assumed to be in the start line."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endColumn")]
    pub end_column: Option<i64>,
    #[doc = " An optional end line of the actual range covered by the breakpoint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endLine")]
    pub end_line: Option<i64>,
    #[doc = " An optional identifier for the breakpoint. It is needed if breakpoint events are used to "]
    #[doc = " update or remove breakpoints."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = " An optional memory reference to where the breakpoint is set."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructionReference")]
    pub instruction_reference: Option<String>,
    #[doc = " The start line of the actual range covered by the breakpoint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[doc = " An optional message about the state of the breakpoint."]
    #[doc = " This is shown to the user and can be used to explain why a breakpoint could not be "]
    #[doc = " verified."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " An optional offset from the instruction reference."]
    #[doc = " This can be negative."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[doc = " The source where the breakpoint is located."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[doc = " If true breakpoint could be set (but not necessarily at the desired location)."]
    pub verified: bool,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BreakpointEventBody {
    #[doc = " The 'id' attribute is used to find the target breakpoint and the other attributes are used "]
    #[doc = " as the new values."]
    pub breakpoint: Breakpoint,
    #[doc = " The reason for the event."]
    pub reason: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BreakpointEvent {
    #[doc = " Event-specific information."]
    pub body: BreakpointEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Properties of a breakpoint location returned from the 'breakpointLocations' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BreakpointLocation {
    #[doc = " Optional start column of breakpoint location."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[doc = " Optional end column of breakpoint location if the location covers a range."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endColumn")]
    pub end_column: Option<i64>,
    #[doc = " Optional end line of breakpoint location if the location covers a range."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endLine")]
    pub end_line: Option<i64>,
    #[doc = " Start line of breakpoint location."]
    pub line: i64,
}
#[doc = " Arguments for 'breakpointLocations' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BreakpointLocationsArguments {
    #[doc = " Optional start column of range to search possible breakpoint locations in. If no start "]
    #[doc = " column is given, the first column in the start line is assumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[doc = " Optional end column of range to search possible breakpoint locations in. If no end column "]
    #[doc = " is given, then it is assumed to be in the last column of the end line."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endColumn")]
    pub end_column: Option<i64>,
    #[doc = " Optional end line of range to search possible breakpoint locations in. If no end line is "]
    #[doc = " given, then the end line is assumed to be the start line."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endLine")]
    pub end_line: Option<i64>,
    #[doc = " Start line of range to search possible breakpoint locations in. If only the line is "]
    #[doc = " specified, the request returns all possible locations in that line."]
    pub line: i64,
    #[doc = " The source location of the breakpoints; either 'source.path' or 'source.reference' must be "]
    #[doc = " specified."]
    pub source: Source,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BreakpointLocationsRequest {
    #[doc = " Object containing arguments for the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<BreakpointLocationsArguments>,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BreakpointLocationsResponseBody {
    #[doc = " Sorted set of possible breakpoint locations."]
    pub breakpoints: Vec<BreakpointLocation>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BreakpointLocationsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: BreakpointLocationsResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'cancel' request."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CancelArguments {
    #[doc = " The ID (attribute 'progressId') of the progress to cancel. If missing no progress is "]
    #[doc = " cancelled."]
    #[doc = " Both a 'requestId' and a 'progressId' can be specified in one request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "progressId")]
    pub progress_id: Option<String>,
    #[doc = " The ID (attribute 'seq') of the request to cancel. If missing no request is cancelled."]
    #[doc = " Both a 'requestId' and a 'progressId' can be specified in one request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestId")]
    pub request_id: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CancelRequest {
    #[doc = " Object containing arguments for the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<CancelArguments>,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CancelResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Information about the capabilities of a debug adapter."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Capabilities {
    #[doc = " The set of additional module information exposed by the debug adapter."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalModuleColumns")]
    pub additional_module_columns: Option<Vec<ColumnDescriptor>>,
    #[doc = " The set of characters that should trigger completion in a REPL. If not specified, the UI "]
    #[doc = " should assume the '.' character."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completionTriggerCharacters")]
    pub completion_trigger_characters: Option<Vec<String>>,
    #[doc = " Available exception filter options for the 'setExceptionBreakpoints' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionBreakpointFilters")]
    pub exception_breakpoint_filters: Option<Vec<ExceptionBreakpointsFilter>>,
    #[doc = " The debug adapter supports the 'suspendDebuggee' attribute on the 'disconnect' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportSuspendDebuggee")]
    pub support_suspend_debuggee: Option<bool>,
    #[doc = " The debug adapter supports the 'terminateDebuggee' attribute on the 'disconnect' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportTerminateDebuggee")]
    pub support_terminate_debuggee: Option<bool>,
    #[doc = " Checksum algorithms supported by the debug adapter."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportedChecksumAlgorithms")]
    pub supported_checksum_algorithms: Option<Vec<ChecksumAlgorithm>>,
    #[doc = " The debug adapter supports the 'breakpointLocations' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsBreakpointLocationsRequest")]
    pub supports_breakpoint_locations_request: Option<bool>,
    #[doc = " The debug adapter supports the 'cancel' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsCancelRequest")]
    pub supports_cancel_request: Option<bool>,
    #[doc = " The debug adapter supports the 'clipboard' context value in the 'evaluate' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsClipboardContext")]
    pub supports_clipboard_context: Option<bool>,
    #[doc = " The debug adapter supports the 'completions' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsCompletionsRequest")]
    pub supports_completions_request: Option<bool>,
    #[doc = " The debug adapter supports conditional breakpoints."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsConditionalBreakpoints")]
    pub supports_conditional_breakpoints: Option<bool>,
    #[doc = " The debug adapter supports the 'configurationDone' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsConfigurationDoneRequest")]
    pub supports_configuration_done_request: Option<bool>,
    #[doc = " The debug adapter supports data breakpoints."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsDataBreakpoints")]
    pub supports_data_breakpoints: Option<bool>,
    #[doc = " The debug adapter supports the delayed loading of parts of the stack, which requires that "]
    #[doc = " both the 'startFrame' and 'levels' arguments and an optional 'totalFrames' result of the "]
    #[doc = " 'StackTrace' request are supported."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsDelayedStackTraceLoading")]
    pub supports_delayed_stack_trace_loading: Option<bool>,
    #[doc = " The debug adapter supports the 'disassemble' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsDisassembleRequest")]
    pub supports_disassemble_request: Option<bool>,
    #[doc = " The debug adapter supports a (side effect free) evaluate request for data hovers."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsEvaluateForHovers")]
    pub supports_evaluate_for_hovers: Option<bool>,
    #[doc = " The debug adapter supports 'filterOptions' as an argument on the 'setExceptionBreakpoints' "]
    #[doc = " request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsExceptionFilterOptions")]
    pub supports_exception_filter_options: Option<bool>,
    #[doc = " The debug adapter supports the 'exceptionInfo' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsExceptionInfoRequest")]
    pub supports_exception_info_request: Option<bool>,
    #[doc = " The debug adapter supports 'exceptionOptions' on the setExceptionBreakpoints request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsExceptionOptions")]
    pub supports_exception_options: Option<bool>,
    #[doc = " The debug adapter supports function breakpoints."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsFunctionBreakpoints")]
    pub supports_function_breakpoints: Option<bool>,
    #[doc = " The debug adapter supports the 'gotoTargets' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsGotoTargetsRequest")]
    pub supports_goto_targets_request: Option<bool>,
    #[doc = " The debug adapter supports breakpoints that break execution after a specified number of "]
    #[doc = " hits."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsHitConditionalBreakpoints")]
    pub supports_hit_conditional_breakpoints: Option<bool>,
    #[doc = " The debug adapter supports adding breakpoints based on instruction references."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsInstructionBreakpoints")]
    pub supports_instruction_breakpoints: Option<bool>,
    #[doc = " The debug adapter supports the 'loadedSources' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsLoadedSourcesRequest")]
    pub supports_loaded_sources_request: Option<bool>,
    #[doc = " The debug adapter supports logpoints by interpreting the 'logMessage' attribute of the "]
    #[doc = " SourceBreakpoint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsLogPoints")]
    pub supports_log_points: Option<bool>,
    #[doc = " The debug adapter supports the 'modules' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsModulesRequest")]
    pub supports_modules_request: Option<bool>,
    #[doc = " The debug adapter supports the 'readMemory' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsReadMemoryRequest")]
    pub supports_read_memory_request: Option<bool>,
    #[doc = " The debug adapter supports restarting a frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsRestartFrame")]
    pub supports_restart_frame: Option<bool>,
    #[doc = " The debug adapter supports the 'restart' request. In this case a client should not "]
    #[doc = " implement 'restart' by terminating and relaunching the adapter but by calling the "]
    #[doc = " RestartRequest."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsRestartRequest")]
    pub supports_restart_request: Option<bool>,
    #[doc = " The debug adapter supports the 'setExpression' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsSetExpression")]
    pub supports_set_expression: Option<bool>,
    #[doc = " The debug adapter supports setting a variable to a value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsSetVariable")]
    pub supports_set_variable: Option<bool>,
    #[doc = " The debug adapter supports the 'singleThread' property on the execution requests "]
    #[doc = " ('continue', 'next', 'stepIn', 'stepOut', 'reverseContinue', 'stepBack')."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsSingleThreadExecutionRequests")]
    pub supports_single_thread_execution_requests: Option<bool>,
    #[doc = " The debug adapter supports stepping back via the 'stepBack' and 'reverseContinue' requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsStepBack")]
    pub supports_step_back: Option<bool>,
    #[doc = " The debug adapter supports the 'stepInTargets' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsStepInTargetsRequest")]
    pub supports_step_in_targets_request: Option<bool>,
    #[doc = " The debug adapter supports stepping granularities (argument 'granularity') for the stepping "]
    #[doc = " requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsSteppingGranularity")]
    pub supports_stepping_granularity: Option<bool>,
    #[doc = " The debug adapter supports the 'terminate' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsTerminateRequest")]
    pub supports_terminate_request: Option<bool>,
    #[doc = " The debug adapter supports the 'terminateThreads' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsTerminateThreadsRequest")]
    pub supports_terminate_threads_request: Option<bool>,
    #[doc = " The debug adapter supports a 'format' attribute on the stackTraceRequest, variablesRequest, "]
    #[doc = " and evaluateRequest."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsValueFormattingOptions")]
    pub supports_value_formatting_options: Option<bool>,
    #[doc = " The debug adapter supports the 'writeMemory' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsWriteMemoryRequest")]
    pub supports_write_memory_request: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CapabilitiesEventBody {
    #[doc = " The set of updated capabilities."]
    pub capabilities: Capabilities,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CapabilitiesEvent {
    #[doc = " Event-specific information."]
    pub body: CapabilitiesEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " The checksum of an item calculated by the specified algorithm."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Checksum {
    #[doc = " The algorithm used to calculate this checksum."]
    pub algorithm: ChecksumAlgorithm,
    #[doc = " Value of the checksum."]
    pub checksum: String,
}
#[doc = " Names of checksum algorithms that may be supported by a debug adapter."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ChecksumAlgorithm {
    #[serde(rename = "MD5")]
    Md5,
    #[serde(rename = "SHA1")]
    Sha1,
    #[serde(rename = "SHA256")]
    Sha256,
    #[serde(rename = "timestamp")]
    Timestamp,
}
#[doc = " A ColumnDescriptor specifies what module attribute to show in a column of the ModulesView, how "]
#[doc = " to format it,"]
#[doc = " and what the column's label should be."]
#[doc = " It is only used if the underlying UI actually supports this level of customization."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ColumnDescriptor {
    #[doc = " Name of the attribute rendered in this column."]
    #[serde(rename = "attributeName")]
    pub attribute_name: String,
    #[doc = " Format to use for the rendered values in this column. TBD how the format strings looks "]
    #[doc = " like."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = " Header UI label of column."]
    pub label: String,
    #[doc = " Datatype of values in this column.  Defaults to 'string' if not specified."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = " Width of this column in characters (hint only)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
#[doc = " CompletionItems are the suggestions returned from the CompletionsRequest."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionItem {
    #[doc = " A human-readable string with additional information about this item, like type or symbol "]
    #[doc = " information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[doc = " The label of this completion item. By default this is also the text that is inserted when "]
    #[doc = " selecting this completion."]
    pub label: String,
    #[doc = " This value determines how many characters are overwritten by the completion text."]
    #[doc = " If missing the value 0 is assumed which results in the completion text being inserted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[doc = " Determines the length of the new selection after the text has been inserted (or replaced)."]
    #[doc = " The selection can not extend beyond the bounds of the completion text."]
    #[doc = " If omitted the length is assumed to be 0."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "selectionLength")]
    pub selection_length: Option<i64>,
    #[doc = " Determines the start of the new selection after the text has been inserted (or replaced)."]
    #[doc = " The start position must in the range 0 and length of the completion text."]
    #[doc = " If omitted the selection starts at the end of the completion text."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "selectionStart")]
    pub selection_start: Option<i64>,
    #[doc = " A string that should be used when comparing this item with other items. When `falsy` the "]
    #[doc = " label is used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sortText")]
    pub sort_text: Option<String>,
    #[doc = " This value determines the location (in the CompletionsRequest's 'text' attribute) where the "]
    #[doc = " completion text is added."]
    #[doc = " If missing the text is added at the location specified by the CompletionsRequest's 'column' "]
    #[doc = " attribute."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[doc = " If text is not falsy then it is inserted instead of the label."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = " The item's type. Typically the client uses this information to render the item in the UI "]
    #[doc = " with an icon."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<CompletionItemType>,
}
#[doc = " Some predefined types for the CompletionItem. Please note that not all clients have specific "]
#[doc = " icons for all of them."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum CompletionItemType {
    #[serde(rename = "method")]
    Method,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "constructor")]
    Constructor,
    #[serde(rename = "field")]
    Field,
    #[serde(rename = "variable")]
    Variable,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "module")]
    Module,
    #[serde(rename = "property")]
    Property,
    #[serde(rename = "unit")]
    Unit,
    #[serde(rename = "value")]
    Value,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "keyword")]
    Keyword,
    #[serde(rename = "snippet")]
    Snippet,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "reference")]
    Reference,
    #[serde(rename = "customcolor")]
    Customcolor,
}
#[doc = " Arguments for 'completions' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionsArguments {
    #[doc = " The character position for which to determine the completion proposals."]
    pub column: i64,
    #[doc = " Returns completions in the scope of this stack frame. If not specified, the completions are "]
    #[doc = " returned for the global scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<i64>,
    #[doc = " An optional line for which to determine the completion proposals. If missing the first line "]
    #[doc = " of the text is assumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[doc = " One or more source lines. Typically this is the text a user has typed into the debug "]
    #[doc = " console before he asked for completion."]
    pub text: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionsRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: CompletionsArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionsResponseBody {
    #[doc = " The possible completions for ."]
    pub targets: Vec<CompletionItem>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: CompletionsResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'configurationDone' request."]
pub type ConfigurationDoneArguments = ::std::collections::BTreeMap<String, serde_json::Value>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ConfigurationDoneRequest {
    #[doc = " Object containing arguments for the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<ConfigurationDoneArguments>,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ConfigurationDoneResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'continue' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ContinueArguments {
    #[doc = " If this optional flag is true, execution is resumed only for the thread with given "]
    #[doc = " 'threadId'."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "singleThread")]
    pub single_thread: Option<bool>,
    #[doc = " Specifies the active thread. If the debug adapter supports single thread execution (see "]
    #[doc = " 'supportsSingleThreadExecutionRequests') and the optional argument 'singleThread' is true, "]
    #[doc = " only the thread with this ID is resumed."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ContinueRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: ContinueArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ContinueResponseBody {
    #[doc = " The value true (or a missing property) signals to the client that all threads have been "]
    #[doc = " resumed. The value false must be returned if not all threads were resumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allThreadsContinued")]
    pub all_threads_continued: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ContinueResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: ContinueResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ContinuedEventBody {
    #[doc = " If 'allThreadsContinued' is true, a debug adapter can announce that all threads have "]
    #[doc = " continued."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allThreadsContinued")]
    pub all_threads_continued: Option<bool>,
    #[doc = " The thread which was continued."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ContinuedEvent {
    #[doc = " Event-specific information."]
    pub body: ContinuedEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Properties of a data breakpoint passed to the setDataBreakpoints request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataBreakpoint {
    #[doc = " The access type of the data."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accessType")]
    pub access_type: Option<DataBreakpointAccessType>,
    #[doc = " An optional expression for conditional breakpoints."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = " An id representing the data. This id is returned from the dataBreakpointInfo request."]
    #[serde(rename = "dataId")]
    pub data_id: String,
    #[doc = " An optional expression that controls how many hits of the breakpoint are ignored."]
    #[doc = " The backend is expected to interpret the expression as needed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hitCondition")]
    pub hit_condition: Option<String>,
}
#[doc = " This enumeration defines all possible access types for data breakpoints."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum DataBreakpointAccessType {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "readWrite")]
    ReadWrite,
}
#[doc = " Arguments for 'dataBreakpointInfo' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataBreakpointInfoArguments {
    #[doc = " The name of the Variable's child to obtain data breakpoint information for."]
    #[doc = " If variablesReference isn't provided, this can be an expression."]
    pub name: String,
    #[doc = " Reference to the Variable container if the data breakpoint is requested for a child of the "]
    #[doc = " container."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "variablesReference")]
    pub variables_reference: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataBreakpointInfoRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: DataBreakpointInfoArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataBreakpointInfoResponseBody {
    #[doc = " Optional attribute listing the available access types for a potential data breakpoint. A UI "]
    #[doc = " frontend could surface this information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accessTypes")]
    pub access_types: Option<Vec<DataBreakpointAccessType>>,
    #[doc = " Optional attribute indicating that a potential data breakpoint could be persisted across "]
    #[doc = " sessions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canPersist")]
    pub can_persist: Option<bool>,
    #[doc = " An identifier for the data on which a data breakpoint can be registered with the "]
    #[doc = " setDataBreakpoints request or null if no data breakpoint is available."]
    #[serde(default)]
    #[serde(rename = "dataId")]
    pub data_id: Option<String>,
    #[doc = " UI string that describes on what data the breakpoint is set on or why a data breakpoint is "]
    #[doc = " not available."]
    pub description: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataBreakpointInfoResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: DataBreakpointInfoResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'disassemble' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DisassembleArguments {
    #[doc = " Number of instructions to disassemble starting at the specified location and offset."]
    #[doc = " An adapter must return exactly this number of instructions - any unavailable instructions "]
    #[doc = " should be replaced with an implementation-defined 'invalid instruction' value."]
    #[serde(rename = "instructionCount")]
    pub instruction_count: i64,
    #[doc = " Optional offset (in instructions) to be applied after the byte offset (if any) before "]
    #[doc = " disassembling. Can be negative."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructionOffset")]
    pub instruction_offset: Option<i64>,
    #[doc = " Memory reference to the base location containing the instructions to disassemble."]
    #[serde(rename = "memoryReference")]
    pub memory_reference: String,
    #[doc = " Optional offset (in bytes) to be applied to the reference location before disassembling. "]
    #[doc = " Can be negative."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[doc = " If true, the adapter should attempt to resolve memory addresses and other values to "]
    #[doc = " symbolic names."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveSymbols")]
    pub resolve_symbols: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DisassembleRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: DisassembleArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DisassembleResponseBody {
    #[doc = " The list of disassembled instructions."]
    pub instructions: Vec<DisassembledInstruction>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DisassembleResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<DisassembleResponseBody>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Represents a single disassembled instruction."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DisassembledInstruction {
    #[doc = " The address of the instruction. Treated as a hex value if prefixed with '0x', or as a "]
    #[doc = " decimal value otherwise."]
    pub address: String,
    #[doc = " The column within the line that corresponds to this instruction, if any."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[doc = " The end column of the range that corresponds to this instruction, if any."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endColumn")]
    pub end_column: Option<i64>,
    #[doc = " The end line of the range that corresponds to this instruction, if any."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endLine")]
    pub end_line: Option<i64>,
    #[doc = " Text representing the instruction and its operands, in an implementation-defined format."]
    pub instruction: String,
    #[doc = " Optional raw bytes representing the instruction and its operands, in an "]
    #[doc = " implementation-defined format."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructionBytes")]
    pub instruction_bytes: Option<String>,
    #[doc = " The line within the source location that corresponds to this instruction, if any."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[doc = " Source location that corresponds to this instruction, if any."]
    #[doc = " Should always be set (if available) on the first instruction returned,"]
    #[doc = " but can be omitted afterwards if this instruction maps to the same source file as the "]
    #[doc = " previous instruction."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Source>,
    #[doc = " Name of the symbol that corresponds with the location of this instruction, if any."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}
#[doc = " Arguments for 'disconnect' request."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DisconnectArguments {
    #[doc = " A value of true indicates that this 'disconnect' request is part of a restart sequence."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,
    #[doc = " Indicates whether the debuggee should stay suspended when the debugger is disconnected."]
    #[doc = " If unspecified, the debuggee should resume execution."]
    #[doc = " The attribute is only honored by a debug adapter if the capability 'supportSuspendDebuggee' "]
    #[doc = " is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suspendDebuggee")]
    pub suspend_debuggee: Option<bool>,
    #[doc = " Indicates whether the debuggee should be terminated when the debugger is disconnected."]
    #[doc = " If unspecified, the debug adapter is free to do whatever it thinks is best."]
    #[doc = " The attribute is only honored by a debug adapter if the capability "]
    #[doc = " 'supportTerminateDebuggee' is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "terminateDebuggee")]
    pub terminate_debuggee: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DisconnectRequest {
    #[doc = " Object containing arguments for the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<DisconnectArguments>,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DisconnectResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ErrorResponseBody {
    #[doc = " An optional, structured error message."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Message>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: ErrorResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'evaluate' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EvaluateArguments {
    #[doc = " The context in which the evaluate request is run."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = " The expression to evaluate."]
    pub expression: String,
    #[doc = " Specifies details on how to format the Evaluate result."]
    #[doc = " The attribute is only honored by a debug adapter if the capability "]
    #[doc = " 'supportsValueFormattingOptions' is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
    #[doc = " Evaluate the expression in the scope of this stack frame. If not specified, the expression "]
    #[doc = " is evaluated in the global scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EvaluateRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: EvaluateArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EvaluateResponseBody {
    #[doc = " The number of indexed child variables."]
    #[doc = " The client can use this optional information to present the variables in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "indexedVariables")]
    pub indexed_variables: Option<i64>,
    #[doc = " Optional memory reference to a location appropriate for this result."]
    #[doc = " For pointer type eval results, this is generally a reference to the memory address "]
    #[doc = " contained in the pointer."]
    #[doc = " This attribute should be returned by a debug adapter if the client has passed the value "]
    #[doc = " true for the 'supportsMemoryReferences' capability of the 'initialize' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "memoryReference")]
    pub memory_reference: Option<String>,
    #[doc = " The number of named child variables."]
    #[doc = " The client can use this optional information to present the variables in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "namedVariables")]
    pub named_variables: Option<i64>,
    #[doc = " Properties of a evaluate result that can be used to determine how to render the result in "]
    #[doc = " the UI."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "presentationHint")]
    pub presentation_hint: Option<VariablePresentationHint>,
    #[doc = " The result of the evaluate request."]
    pub result: String,
    #[doc = " The optional type of the evaluate result."]
    #[doc = " This attribute should only be returned by a debug adapter if the client has passed the "]
    #[doc = " value true for the 'supportsVariableType' capability of the 'initialize' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = " If variablesReference is > 0, the evaluate result is structured and its children can be "]
    #[doc = " retrieved by passing variablesReference to the VariablesRequest."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(rename = "variablesReference")]
    pub variables_reference: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EvaluateResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: EvaluateResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Event {
    #[doc = " Event-specific information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " This enumeration defines all possible conditions when a thrown exception should result in a "]
#[doc = " break."]
#[doc = " never: never breaks,"]
#[doc = " always: always breaks,"]
#[doc = " unhandled: breaks when exception unhandled,"]
#[doc = " userUnhandled: breaks if the exception is not handled by user code."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ExceptionBreakMode {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "unhandled")]
    Unhandled,
    #[serde(rename = "userUnhandled")]
    UserUnhandled,
}
#[doc = " An ExceptionBreakpointsFilter is shown in the UI as an filter option for configuring how "]
#[doc = " exceptions are dealt with."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExceptionBreakpointsFilter {
    #[doc = " An optional help text providing information about the condition. This string is shown as "]
    #[doc = " the placeholder text for a text box and must be translated."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conditionDescription")]
    pub condition_description: Option<String>,
    #[doc = " Initial value of the filter option. If not specified a value 'false' is assumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[doc = " An optional help text providing additional information about the exception filter. This "]
    #[doc = " string is typically shown as a hover and must be translated."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = " The internal ID of the filter option. This value is passed to the 'setExceptionBreakpoints' "]
    #[doc = " request."]
    pub filter: String,
    #[doc = " The name of the filter option. This will be shown in the UI."]
    pub label: String,
    #[doc = " Controls whether a condition can be specified for this filter option. If false or missing, "]
    #[doc = " a condition can not be set."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsCondition")]
    pub supports_condition: Option<bool>,
}
#[doc = " Detailed information about an exception that has occurred."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ExceptionDetails {
    #[doc = " Optional expression that can be evaluated in the current scope to obtain the exception "]
    #[doc = " object."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "evaluateName")]
    pub evaluate_name: Option<String>,
    #[doc = " Fully-qualified type name of the exception object."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fullTypeName")]
    pub full_type_name: Option<String>,
    #[doc = " Details of the exception contained by this exception, if any."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "innerException")]
    pub inner_exception: Option<Vec<ExceptionDetails>>,
    #[doc = " Message contained in the exception."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Stack trace at the time the exception was thrown."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stackTrace")]
    pub stack_trace: Option<String>,
    #[doc = " Short type name of the exception object."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "typeName")]
    pub type_name: Option<String>,
}
#[doc = " An ExceptionFilterOptions is used to specify an exception filter together with a condition for "]
#[doc = " the 'setExceptionBreakpoints' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExceptionFilterOptions {
    #[doc = " An optional expression for conditional exceptions."]
    #[doc = " The exception will break into the debugger if the result of the condition is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = " ID of an exception filter returned by the 'exceptionBreakpointFilters' capability."]
    #[serde(rename = "filterId")]
    pub filter_id: String,
}
#[doc = " Arguments for 'exceptionInfo' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExceptionInfoArguments {
    #[doc = " Thread for which exception information should be retrieved."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExceptionInfoRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: ExceptionInfoArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExceptionInfoResponseBody {
    #[doc = " Mode that caused the exception notification to be raised."]
    #[serde(rename = "breakMode")]
    pub break_mode: ExceptionBreakMode,
    #[doc = " Descriptive text for the exception provided by the debug adapter."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = " Detailed information about the exception."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ExceptionDetails>,
    #[doc = " ID of the exception that was thrown."]
    #[serde(rename = "exceptionId")]
    pub exception_id: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExceptionInfoResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: ExceptionInfoResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " An ExceptionOptions assigns configuration options to a set of exceptions."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExceptionOptions {
    #[doc = " Condition when a thrown exception should result in a break."]
    #[serde(rename = "breakMode")]
    pub break_mode: ExceptionBreakMode,
    #[doc = " A path that selects a single or multiple exceptions in a tree. If 'path' is missing, the "]
    #[doc = " whole tree is selected."]
    #[doc = " By convention the first segment of the path is a category that is used to group exceptions "]
    #[doc = " in the UI."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<ExceptionPathSegment>>,
}
#[doc = " An ExceptionPathSegment represents a segment in a path that is used to match leafs or nodes in "]
#[doc = " a tree of exceptions."]
#[doc = " If a segment consists of more than one name, it matches the names provided if 'negate' is false "]
#[doc = " or missing or"]
#[doc = " it matches anything except the names provided if 'negate' is true."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExceptionPathSegment {
    #[doc = " Depending on the value of 'negate' the names that should match or not match."]
    pub names: Vec<String>,
    #[doc = " If false or missing this segment matches the names provided, otherwise it matches anything "]
    #[doc = " except the names provided."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExitedEventBody {
    #[doc = " The exit code returned from the debuggee."]
    #[serde(rename = "exitCode")]
    pub exit_code: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExitedEvent {
    #[doc = " Event-specific information."]
    pub body: ExitedEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Properties of a breakpoint passed to the setFunctionBreakpoints request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FunctionBreakpoint {
    #[doc = " An optional expression for conditional breakpoints."]
    #[doc = " It is only honored by a debug adapter if the capability 'supportsConditionalBreakpoints' is "]
    #[doc = " true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = " An optional expression that controls how many hits of the breakpoint are ignored."]
    #[doc = " The backend is expected to interpret the expression as needed."]
    #[doc = " The attribute is only honored by a debug adapter if the capability "]
    #[doc = " 'supportsHitConditionalBreakpoints' is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hitCondition")]
    pub hit_condition: Option<String>,
    #[doc = " The name of the function."]
    pub name: String,
}
#[doc = " Arguments for 'goto' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GotoArguments {
    #[doc = " The location where the debuggee will continue to run."]
    #[serde(rename = "targetId")]
    pub target_id: i64,
    #[doc = " Set the goto target for this thread."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GotoRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: GotoArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GotoResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " A GotoTarget describes a code location that can be used as a target in the 'goto' request."]
#[doc = " The possible goto targets can be determined via the 'gotoTargets' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GotoTarget {
    #[doc = " An optional column of the goto target."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[doc = " An optional end column of the range covered by the goto target."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endColumn")]
    pub end_column: Option<i64>,
    #[doc = " An optional end line of the range covered by the goto target."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endLine")]
    pub end_line: Option<i64>,
    #[doc = " Unique identifier for a goto target. This is used in the goto request."]
    pub id: i64,
    #[doc = " Optional memory reference for the instruction pointer value represented by this target."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructionPointerReference")]
    pub instruction_pointer_reference: Option<String>,
    #[doc = " The name of the goto target (shown in the UI)."]
    pub label: String,
    #[doc = " The line of the goto target."]
    pub line: i64,
}
#[doc = " Arguments for 'gotoTargets' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GotoTargetsArguments {
    #[doc = " An optional column location for which the goto targets are determined."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[doc = " The line location for which the goto targets are determined."]
    pub line: i64,
    #[doc = " The source location for which the goto targets are determined."]
    pub source: Source,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GotoTargetsRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: GotoTargetsArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GotoTargetsResponseBody {
    #[doc = " The possible goto targets of the specified location."]
    pub targets: Vec<GotoTarget>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GotoTargetsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: GotoTargetsResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: InitializeRequestArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'initialize' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeRequestArguments {
    #[doc = " The ID of the debug adapter."]
    #[serde(rename = "adapterID")]
    pub adapter_id: String,
    #[doc = " The ID of the (frontend) client using this adapter."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clientID")]
    pub client_id: Option<String>,
    #[doc = " The human readable name of the (frontend) client using this adapter."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clientName")]
    pub client_name: Option<String>,
    #[doc = " If true all column numbers are 1-based (default)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "columnsStartAt1")]
    pub columns_start_at_1: Option<bool>,
    #[doc = " If true all line numbers are 1-based (default)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "linesStartAt1")]
    pub lines_start_at_1: Option<bool>,
    #[doc = " The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US or de-CH."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[doc = " Determines in what format paths are specified. The default is 'path', which is the native "]
    #[doc = " format."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pathFormat")]
    pub path_format: Option<String>,
    #[doc = " Client supports the invalidated event."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsInvalidatedEvent")]
    pub supports_invalidated_event: Option<bool>,
    #[doc = " Client supports the memory event."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsMemoryEvent")]
    pub supports_memory_event: Option<bool>,
    #[doc = " Client supports memory references."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsMemoryReferences")]
    pub supports_memory_references: Option<bool>,
    #[doc = " Client supports progress reporting."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsProgressReporting")]
    pub supports_progress_reporting: Option<bool>,
    #[doc = " Client supports the runInTerminal request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsRunInTerminalRequest")]
    pub supports_run_in_terminal_request: Option<bool>,
    #[doc = " Client supports the paging of variables."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsVariablePaging")]
    pub supports_variable_paging: Option<bool>,
    #[doc = " Client supports the optional type attribute for variables."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supportsVariableType")]
    pub supports_variable_type: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeResponse {
    #[doc = " The capabilities of this debug adapter."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Capabilities>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializedEvent {
    #[doc = " Event-specific information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Properties of a breakpoint passed to the setInstructionBreakpoints request"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InstructionBreakpoint {
    #[doc = " An optional expression for conditional breakpoints."]
    #[doc = " It is only honored by a debug adapter if the capability 'supportsConditionalBreakpoints' is "]
    #[doc = " true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = " An optional expression that controls how many hits of the breakpoint are ignored."]
    #[doc = " The backend is expected to interpret the expression as needed."]
    #[doc = " The attribute is only honored by a debug adapter if the capability "]
    #[doc = " 'supportsHitConditionalBreakpoints' is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hitCondition")]
    pub hit_condition: Option<String>,
    #[doc = " The instruction reference of the breakpoint."]
    #[doc = " This should be a memory or instruction pointer reference from an EvaluateResponse, "]
    #[doc = " Variable, StackFrame, GotoTarget, or Breakpoint."]
    #[serde(rename = "instructionReference")]
    pub instruction_reference: String,
    #[doc = " An optional offset from the instruction reference."]
    #[doc = " This can be negative."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
#[doc = " Logical areas that can be invalidated by the 'invalidated' event."]
pub type InvalidatedAreas = String;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct InvalidatedEventBody {
    #[doc = " Optional set of logical areas that got invalidated. This property has a hint "]
    #[doc = " characteristic: a client can only be expected to make a 'best effort' in honouring the "]
    #[doc = " areas but there are no guarantees. If this property is missing, empty, or if values are not "]
    #[doc = " understand the client should assume a single value 'all'."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<Vec<InvalidatedAreas>>,
    #[doc = " If specified, the client only needs to refetch data related to this stack frame (and the "]
    #[doc = " 'threadId' is ignored)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stackFrameId")]
    pub stack_frame_id: Option<i64>,
    #[doc = " If specified, the client only needs to refetch data related to this thread."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threadId")]
    pub thread_id: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InvalidatedEvent {
    #[doc = " Event-specific information."]
    pub body: InvalidatedEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LaunchRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: LaunchRequestArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'launch' request. Additional attributes are implementation specific."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct LaunchRequestArguments {
    #[doc = " Optional data from the previous, restarted session."]
    #[doc = " The data is sent as the 'restart' attribute of the 'terminated' event."]
    #[doc = " The client should leave the data intact."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "__restart")]
    pub _restart: Option<serde_json::Value>,
    #[doc = " If noDebug is true the launch request should launch the program without enabling debugging."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "noDebug")]
    pub no_debug: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LaunchResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LoadedSourceEventBody {
    #[doc = " The reason for the event."]
    pub reason: String,
    #[doc = " The new, changed, or removed source."]
    pub source: Source,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LoadedSourceEvent {
    #[doc = " Event-specific information."]
    pub body: LoadedSourceEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'loadedSources' request."]
pub type LoadedSourcesArguments = ::std::collections::BTreeMap<String, serde_json::Value>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LoadedSourcesRequest {
    #[doc = " Object containing arguments for the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<LoadedSourcesArguments>,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LoadedSourcesResponseBody {
    #[doc = " Set of loaded sources."]
    pub sources: Vec<Source>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LoadedSourcesResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: LoadedSourcesResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct MemoryEventBody {
    #[doc = " Number of bytes updated."]
    pub count: i64,
    #[doc = " Memory reference of a memory range that has been updated."]
    #[serde(rename = "memoryReference")]
    pub memory_reference: String,
    #[doc = " Starting offset in bytes where memory has been updated. Can be negative."]
    pub offset: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct MemoryEvent {
    #[doc = " Event-specific information."]
    pub body: MemoryEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " A structured message object. Used to return errors from requests."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Message {
    #[doc = " A format string for the message. Embedded variables have the form '{name}'."]
    #[doc = " If variable name starts with an underscore character, the variable does not contain user "]
    #[doc = " data (PII) and can be safely used for telemetry purposes."]
    pub format: String,
    #[doc = " Unique identifier for the message."]
    pub id: i64,
    #[doc = " If true send to telemetry."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sendTelemetry")]
    pub send_telemetry: Option<bool>,
    #[doc = " If true show user."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "showUser")]
    pub show_user: Option<bool>,
    #[doc = " An optional url where additional information about this message can be found."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = " An optional label that is presented to the user as the UI for opening the url."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "urlLabel")]
    pub url_label: Option<String>,
    #[doc = " An object used as a dictionary for looking up the variables in the format string."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::BTreeMap<String, String>>,
}
#[doc = " A Module object represents a row in the modules view."]
#[doc = " Two attributes are mandatory: an id identifies a module in the modules view and is used in a "]
#[doc = " ModuleEvent for identifying a module for adding, updating or deleting."]
#[doc = " The name is used to minimally render the module in the UI."]
#[doc = " "]
#[doc = " Additional attributes can be added to the module. They will show up in the module View if they "]
#[doc = " have a corresponding ColumnDescriptor."]
#[doc = " "]
#[doc = " To avoid an unnecessary proliferation of additional attributes with similar semantics but "]
#[doc = " different names"]
#[doc = " we recommend to re-use attributes from the 'recommended' list below first, and only introduce "]
#[doc = " new attributes if nothing appropriate could be found."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Module {
    #[doc = " Address range covered by this module."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "addressRange")]
    pub address_range: Option<String>,
    #[doc = " Module created or modified."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dateTimeStamp")]
    pub date_time_stamp: Option<String>,
    #[doc = " Unique identifier for the module."]
    pub id: serde_json::Value,
    #[doc = " True if the module is optimized."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isOptimized")]
    pub is_optimized: Option<bool>,
    #[doc = " True if the module is considered 'user code' by a debugger that supports 'Just My Code'."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isUserCode")]
    pub is_user_code: Option<bool>,
    #[doc = " A name of the module."]
    pub name: String,
    #[doc = " optional but recommended attributes."]
    #[doc = " always try to use these first before introducing additional attributes."]
    #[doc = " "]
    #[doc = " Logical full path to the module. The exact definition is implementation defined, but "]
    #[doc = " usually this would be a full path to the on-disk file for the module."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = " Logical full path to the symbol file. The exact definition is implementation defined."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "symbolFilePath")]
    pub symbol_file_path: Option<String>,
    #[doc = " User understandable description of if symbols were found for the module (ex: 'Symbols "]
    #[doc = " Loaded', 'Symbols not found', etc."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "symbolStatus")]
    pub symbol_status: Option<String>,
    #[doc = " Version of Module."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ModuleEventBody {
    #[doc = " The new, changed, or removed module. In case of 'removed' only the module id is used."]
    pub module: Module,
    #[doc = " The reason for the event."]
    pub reason: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ModuleEvent {
    #[doc = " Event-specific information."]
    pub body: ModuleEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'modules' request."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ModulesArguments {
    #[doc = " The number of modules to return. If moduleCount is not specified or 0, all modules are "]
    #[doc = " returned."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "moduleCount")]
    pub module_count: Option<i64>,
    #[doc = " The index of the first module to return; if omitted modules start at 0."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startModule")]
    pub start_module: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ModulesRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: ModulesArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ModulesResponseBody {
    #[doc = " All modules or range of modules."]
    pub modules: Vec<Module>,
    #[doc = " The total number of modules available."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalModules")]
    pub total_modules: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ModulesResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: ModulesResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " The ModulesViewDescriptor is the container for all declarative configuration options of a "]
#[doc = " ModuleView."]
#[doc = " For now it only specifies the columns to be shown in the modules view."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ModulesViewDescriptor {
    pub columns: Vec<ColumnDescriptor>,
}
#[doc = " Arguments for 'next' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NextArguments {
    #[doc = " Optional granularity to step. If no granularity is specified, a granularity of 'statement' "]
    #[doc = " is assumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<SteppingGranularity>,
    #[doc = " If this optional flag is true, all other suspended threads are not resumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "singleThread")]
    pub single_thread: Option<bool>,
    #[doc = " Specifies the thread for which to resume execution for one step (of the given granularity)."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NextRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: NextArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NextResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct OutputEventBody {
    #[doc = " The output category. If not specified or if the category is not understand by the client, "]
    #[doc = " 'console' is assumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = " An optional source location column where the output was produced."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[doc = " Optional data to report. For the 'telemetry' category the data will be sent to telemetry, "]
    #[doc = " for the other categories the data is shown in JSON format."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " Support for keeping an output log organized by grouping related messages."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[doc = " An optional source location line where the output was produced."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[doc = " The output to report."]
    pub output: String,
    #[doc = " An optional source location where the output was produced."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[doc = " If an attribute 'variablesReference' exists and its value is > 0, the output contains "]
    #[doc = " objects which can be retrieved by passing 'variablesReference' to the 'variables' request. "]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "variablesReference")]
    pub variables_reference: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct OutputEvent {
    #[doc = " Event-specific information."]
    pub body: OutputEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'pause' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PauseArguments {
    #[doc = " Pause execution for this thread."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PauseRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: PauseArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PauseResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProcessEventBody {
    #[doc = " If true, the process is running on the same computer as the debug adapter."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isLocalProcess")]
    pub is_local_process: Option<bool>,
    #[doc = " The logical name of the process. This is usually the full path to process's executable "]
    #[doc = " file. Example: /home/example/myproj/program.js."]
    pub name: String,
    #[doc = " The size of a pointer or address for this process, in bits. This value may be used by "]
    #[doc = " clients when formatting addresses for display."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pointerSize")]
    pub pointer_size: Option<i64>,
    #[doc = " Describes how the debug engine started debugging this process."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startMethod")]
    pub start_method: Option<String>,
    #[doc = " The system process id of the debugged process. This property will be missing for non-system "]
    #[doc = " processes."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "systemProcessId")]
    pub system_process_id: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProcessEvent {
    #[doc = " Event-specific information."]
    pub body: ProcessEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProgressEndEventBody {
    #[doc = " Optional, more detailed progress message. If omitted, the previous message (if any) is "]
    #[doc = " used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " The ID that was introduced in the initial 'ProgressStartEvent'."]
    #[serde(rename = "progressId")]
    pub progress_id: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProgressEndEvent {
    #[doc = " Event-specific information."]
    pub body: ProgressEndEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProgressStartEventBody {
    #[doc = " If true, the request that reports progress may be canceled with a 'cancel' request."]
    #[doc = " So this property basically controls whether the client should use UX that supports "]
    #[doc = " cancellation."]
    #[doc = " Clients that don't support cancellation are allowed to ignore the setting."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    #[doc = " Optional, more detailed progress message."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Optional progress percentage to display (value range: 0 to 100). If omitted no percentage "]
    #[doc = " will be shown."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    #[doc = " An ID that must be used in subsequent 'progressUpdate' and 'progressEnd' events to make "]
    #[doc = " them refer to the same progress reporting."]
    #[doc = " IDs must be unique within a debug session."]
    #[serde(rename = "progressId")]
    pub progress_id: String,
    #[doc = " The request ID that this progress report is related to. If specified a debug adapter is "]
    #[doc = " expected to emit"]
    #[doc = " progress events for the long running request until the request has been either completed or "]
    #[doc = " cancelled."]
    #[doc = " If the request ID is omitted, the progress report is assumed to be related to some general "]
    #[doc = " activity of the debug adapter."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestId")]
    pub request_id: Option<i64>,
    #[doc = " Mandatory (short) title of the progress reporting. Shown in the UI to describe the long "]
    #[doc = " running operation."]
    pub title: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProgressStartEvent {
    #[doc = " Event-specific information."]
    pub body: ProgressStartEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProgressUpdateEventBody {
    #[doc = " Optional, more detailed progress message. If omitted, the previous message (if any) is "]
    #[doc = " used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Optional progress percentage to display (value range: 0 to 100). If omitted no percentage "]
    #[doc = " will be shown."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    #[doc = " The ID that was introduced in the initial 'progressStart' event."]
    #[serde(rename = "progressId")]
    pub progress_id: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProgressUpdateEvent {
    #[doc = " Event-specific information."]
    pub body: ProgressUpdateEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Base class of requests, responses, and events."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProtocolMessage {
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'readMemory' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReadMemoryArguments {
    #[doc = " Number of bytes to read at the specified location and offset."]
    pub count: i64,
    #[doc = " Memory reference to the base location from which data should be read."]
    #[serde(rename = "memoryReference")]
    pub memory_reference: String,
    #[doc = " Optional offset (in bytes) to be applied to the reference location before reading data. Can "]
    #[doc = " be negative."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReadMemoryRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: ReadMemoryArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReadMemoryResponseBody {
    #[doc = " The address of the first byte of data returned."]
    #[doc = " Treated as a hex value if prefixed with '0x', or as a decimal value otherwise."]
    pub address: String,
    #[doc = " The bytes read from memory, encoded using base64."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[doc = " The number of unreadable bytes encountered after the last successfully read byte."]
    #[doc = " This can be used to determine the number of bytes that must be skipped before a subsequent "]
    #[doc = " 'readMemory' request will succeed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unreadableBytes")]
    pub unreadable_bytes: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReadMemoryResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ReadMemoryResponseBody>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Request {
    #[doc = " Object containing arguments for the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Response {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RestartArgumentsArguments {
    Variant0(LaunchRequestArguments),
    Variant1(AttachRequestArguments),
}
#[doc = " Arguments for 'restart' request."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct RestartArguments {
    #[doc = " The latest version of the 'launch' or 'attach' configuration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<RestartArgumentsArguments>,
}
#[doc = " Arguments for 'restartFrame' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RestartFrameArguments {
    #[doc = " Restart this stackframe."]
    #[serde(rename = "frameId")]
    pub frame_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RestartFrameRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: RestartFrameArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RestartFrameResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RestartRequest {
    #[doc = " Object containing arguments for the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<RestartArguments>,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RestartResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'reverseContinue' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReverseContinueArguments {
    #[doc = " If this optional flag is true, backward execution is resumed only for the thread with given "]
    #[doc = " 'threadId'."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "singleThread")]
    pub single_thread: Option<bool>,
    #[doc = " Specifies the active thread. If the debug adapter supports single thread execution (see "]
    #[doc = " 'supportsSingleThreadExecutionRequests') and the optional argument 'singleThread' is true, "]
    #[doc = " only the thread with this ID is resumed."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReverseContinueRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: ReverseContinueArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReverseContinueResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RunInTerminalRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: RunInTerminalRequestArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'runInTerminal' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RunInTerminalRequestArguments {
    #[doc = " List of arguments. The first argument is the command to run."]
    pub args: Vec<String>,
    #[doc = " Working directory for the command. For non-empty, valid paths this typically results in "]
    #[doc = " execution of a change directory command."]
    pub cwd: String,
    #[doc = " Environment key-value pairs that are added to or removed from the default environment."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<::std::collections::BTreeMap<String, Option<String>>>,
    #[doc = " What kind of terminal to launch."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = " Optional title of the terminal."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct RunInTerminalResponseBody {
    #[doc = " The process ID. The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "processId")]
    pub process_id: Option<i64>,
    #[doc = " The process ID of the terminal shell. The value should be less than or equal to 2147483647 "]
    #[doc = " (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shellProcessId")]
    pub shell_process_id: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RunInTerminalResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: RunInTerminalResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " A Scope is a named container for variables. Optionally a scope can map to a source or a range "]
#[doc = " within a source."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Scope {
    #[doc = " Optional start column of the range covered by this scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[doc = " Optional end column of the range covered by this scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endColumn")]
    pub end_column: Option<i64>,
    #[doc = " Optional end line of the range covered by this scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endLine")]
    pub end_line: Option<i64>,
    #[doc = " If true, the number of variables in this scope is large or expensive to retrieve."]
    pub expensive: bool,
    #[doc = " The number of indexed variables in this scope."]
    #[doc = " The client can use this optional information to present the variables in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "indexedVariables")]
    pub indexed_variables: Option<i64>,
    #[doc = " Optional start line of the range covered by this scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[doc = " Name of the scope such as 'Arguments', 'Locals', or 'Registers'. This string is shown in "]
    #[doc = " the UI as is and can be translated."]
    pub name: String,
    #[doc = " The number of named variables in this scope."]
    #[doc = " The client can use this optional information to present the variables in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "namedVariables")]
    pub named_variables: Option<i64>,
    #[doc = " An optional hint for how to present this scope in the UI. If this attribute is missing, the "]
    #[doc = " scope is shown with a generic UI."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "presentationHint")]
    pub presentation_hint: Option<String>,
    #[doc = " Optional source for this scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[doc = " The variables of this scope can be retrieved by passing the value of variablesReference to "]
    #[doc = " the VariablesRequest."]
    #[serde(rename = "variablesReference")]
    pub variables_reference: i64,
}
#[doc = " Arguments for 'scopes' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ScopesArguments {
    #[doc = " Retrieve the scopes for this stackframe."]
    #[serde(rename = "frameId")]
    pub frame_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ScopesRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: ScopesArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ScopesResponseBody {
    #[doc = " The scopes of the stackframe. If the array has length zero, there are no scopes available."]
    pub scopes: Vec<Scope>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ScopesResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: ScopesResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'setBreakpoints' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetBreakpointsArguments {
    #[doc = " The code locations of the breakpoints."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoints: Option<Vec<SourceBreakpoint>>,
    #[doc = " Deprecated: The code locations of the breakpoints."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<i64>>,
    #[doc = " The source location of the breakpoints; either 'source.path' or 'source.reference' must be "]
    #[doc = " specified."]
    pub source: Source,
    #[doc = " A value of true indicates that the underlying source has been modified which results in new "]
    #[doc = " breakpoint locations."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceModified")]
    pub source_modified: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetBreakpointsRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: SetBreakpointsArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetBreakpointsResponseBody {
    #[doc = " Information about the breakpoints."]
    #[doc = " The array elements are in the same order as the elements of the 'breakpoints' (or the "]
    #[doc = " deprecated 'lines') array in the arguments."]
    pub breakpoints: Vec<Breakpoint>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetBreakpointsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: SetBreakpointsResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'setDataBreakpoints' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetDataBreakpointsArguments {
    #[doc = " The contents of this array replaces all existing data breakpoints. An empty array clears "]
    #[doc = " all data breakpoints."]
    pub breakpoints: Vec<DataBreakpoint>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetDataBreakpointsRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: SetDataBreakpointsArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetDataBreakpointsResponseBody {
    #[doc = " Information about the data breakpoints. The array elements correspond to the elements of "]
    #[doc = " the input argument 'breakpoints' array."]
    pub breakpoints: Vec<Breakpoint>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetDataBreakpointsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: SetDataBreakpointsResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'setExceptionBreakpoints' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetExceptionBreakpointsArguments {
    #[doc = " Configuration options for selected exceptions."]
    #[doc = " The attribute is only honored by a debug adapter if the capability "]
    #[doc = " 'supportsExceptionOptions' is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionOptions")]
    pub exception_options: Option<Vec<ExceptionOptions>>,
    #[doc = " Set of exception filters and their options. The set of all possible exception filters is "]
    #[doc = " defined by the 'exceptionBreakpointFilters' capability. This attribute is only honored by a "]
    #[doc = " debug adapter if the capability 'supportsExceptionFilterOptions' is true. The 'filter' and "]
    #[doc = " 'filterOptions' sets are additive."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filterOptions")]
    pub filter_options: Option<Vec<ExceptionFilterOptions>>,
    #[doc = " Set of exception filters specified by their ID. The set of all possible exception filters "]
    #[doc = " is defined by the 'exceptionBreakpointFilters' capability. The 'filter' and 'filterOptions' "]
    #[doc = " sets are additive."]
    pub filters: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetExceptionBreakpointsRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: SetExceptionBreakpointsArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SetExceptionBreakpointsResponseBody {
    #[doc = " Information about the exception breakpoints or filters."]
    #[doc = " The breakpoints returned are in the same order as the elements of the 'filters', "]
    #[doc = " 'filterOptions', 'exceptionOptions' arrays in the arguments. If both 'filters' and "]
    #[doc = " 'filterOptions' are given, the returned array must start with 'filters' information first, "]
    #[doc = " followed by 'filterOptions' information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoints: Option<Vec<Breakpoint>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetExceptionBreakpointsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<SetExceptionBreakpointsResponseBody>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'setExpression' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetExpressionArguments {
    #[doc = " The l-value expression to assign to."]
    pub expression: String,
    #[doc = " Specifies how the resulting value should be formatted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
    #[doc = " Evaluate the expressions in the scope of this stack frame. If not specified, the "]
    #[doc = " expressions are evaluated in the global scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<i64>,
    #[doc = " The value expression to assign to the l-value expression."]
    pub value: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetExpressionRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: SetExpressionArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetExpressionResponseBody {
    #[doc = " The number of indexed child variables."]
    #[doc = " The client can use this optional information to present the variables in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "indexedVariables")]
    pub indexed_variables: Option<i64>,
    #[doc = " The number of named child variables."]
    #[doc = " The client can use this optional information to present the variables in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "namedVariables")]
    pub named_variables: Option<i64>,
    #[doc = " Properties of a value that can be used to determine how to render the result in the UI."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "presentationHint")]
    pub presentation_hint: Option<VariablePresentationHint>,
    #[doc = " The optional type of the value."]
    #[doc = " This attribute should only be returned by a debug adapter if the client has passed the "]
    #[doc = " value true for the 'supportsVariableType' capability of the 'initialize' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = " The new value of the expression."]
    pub value: String,
    #[doc = " If variablesReference is > 0, the value is structured and its children can be retrieved by "]
    #[doc = " passing variablesReference to the VariablesRequest."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "variablesReference")]
    pub variables_reference: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetExpressionResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: SetExpressionResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'setFunctionBreakpoints' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetFunctionBreakpointsArguments {
    #[doc = " The function names of the breakpoints."]
    pub breakpoints: Vec<FunctionBreakpoint>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetFunctionBreakpointsRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: SetFunctionBreakpointsArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetFunctionBreakpointsResponseBody {
    #[doc = " Information about the breakpoints. The array elements correspond to the elements of the "]
    #[doc = " 'breakpoints' array."]
    pub breakpoints: Vec<Breakpoint>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetFunctionBreakpointsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: SetFunctionBreakpointsResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'setInstructionBreakpoints' request"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetInstructionBreakpointsArguments {
    #[doc = " The instruction references of the breakpoints"]
    pub breakpoints: Vec<InstructionBreakpoint>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetInstructionBreakpointsRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: SetInstructionBreakpointsArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetInstructionBreakpointsResponseBody {
    #[doc = " Information about the breakpoints. The array elements correspond to the elements of the "]
    #[doc = " 'breakpoints' array."]
    pub breakpoints: Vec<Breakpoint>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetInstructionBreakpointsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: SetInstructionBreakpointsResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'setVariable' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetVariableArguments {
    #[doc = " Specifies details on how to format the response value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
    #[doc = " The name of the variable in the container."]
    pub name: String,
    #[doc = " The value of the variable."]
    pub value: String,
    #[doc = " The reference of the variable container."]
    #[serde(rename = "variablesReference")]
    pub variables_reference: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetVariableRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: SetVariableArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetVariableResponseBody {
    #[doc = " The number of indexed child variables."]
    #[doc = " The client can use this optional information to present the variables in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "indexedVariables")]
    pub indexed_variables: Option<i64>,
    #[doc = " The number of named child variables."]
    #[doc = " The client can use this optional information to present the variables in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "namedVariables")]
    pub named_variables: Option<i64>,
    #[doc = " The type of the new value. Typically shown in the UI when hovering over the value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = " The new value of the variable."]
    pub value: String,
    #[doc = " If variablesReference is > 0, the new value is structured and its children can be retrieved "]
    #[doc = " by passing variablesReference to the VariablesRequest."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "variablesReference")]
    pub variables_reference: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetVariableResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: SetVariableResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " A Source is a descriptor for source code."]
#[doc = " It is returned from the debug adapter as part of a StackFrame and it is used by clients when "]
#[doc = " specifying breakpoints."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Source {
    #[doc = " Optional data that a debug adapter might want to loop through the client."]
    #[doc = " The client should leave the data intact and persist it across sessions. The client should "]
    #[doc = " not interpret the data."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "adapterData")]
    pub adapter_data: Option<serde_json::Value>,
    #[doc = " The checksums associated with this file."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksums: Option<Vec<Checksum>>,
    #[doc = " The short name of the source. Every source returned from the debug adapter has a name."]
    #[doc = " When sending a source to the debug adapter this name is optional."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = " The (optional) origin of this source: possible values 'internal module', 'inlined content "]
    #[doc = " from source map', etc."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = " The path of the source to be shown in the UI."]
    #[doc = " It is only used to locate and load the content of the source if no sourceReference is "]
    #[doc = " specified (or its value is 0)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = " An optional hint for how to present the source in the UI."]
    #[doc = " A value of 'deemphasize' can be used to indicate that the source is not available or that "]
    #[doc = " it is skipped on stepping."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "presentationHint")]
    pub presentation_hint: Option<String>,
    #[doc = " If sourceReference > 0 the contents of the source must be retrieved through the "]
    #[doc = " SourceRequest (even if a path is specified)."]
    #[doc = " A sourceReference is only valid for a session, so it must not be used to persist a source."]
    #[doc = " The value should be less than or equal to 2147483647 (2^31-1)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceReference")]
    pub source_reference: Option<i64>,
    #[doc = " An optional list of sources that are related to this source. These may be the source that "]
    #[doc = " generated this source."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
}
#[doc = " Arguments for 'source' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SourceArguments {
    #[doc = " Specifies the source content to load. Either source.path or source.sourceReference must be "]
    #[doc = " specified."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[doc = " The reference to the source. This is the same as source.sourceReference."]
    #[doc = " This is provided for backward compatibility since old backends do not understand the "]
    #[doc = " 'source' attribute."]
    #[serde(rename = "sourceReference")]
    pub source_reference: i64,
}
#[doc = " Properties of a breakpoint or logpoint passed to the setBreakpoints request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SourceBreakpoint {
    #[doc = " An optional source column of the breakpoint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[doc = " An optional expression for conditional breakpoints."]
    #[doc = " It is only honored by a debug adapter if the capability 'supportsConditionalBreakpoints' is "]
    #[doc = " true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = " An optional expression that controls how many hits of the breakpoint are ignored."]
    #[doc = " The backend is expected to interpret the expression as needed."]
    #[doc = " The attribute is only honored by a debug adapter if the capability "]
    #[doc = " 'supportsHitConditionalBreakpoints' is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hitCondition")]
    pub hit_condition: Option<String>,
    #[doc = " The source line of the breakpoint or logpoint."]
    pub line: i64,
    #[doc = " If this attribute exists and is non-empty, the backend must not 'break' (stop)"]
    #[doc = " but log the message instead. Expressions within {} are interpolated."]
    #[doc = " The attribute is only honored by a debug adapter if the capability 'supportsLogPoints' is "]
    #[doc = " true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logMessage")]
    pub log_message: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SourceRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: SourceArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SourceResponseBody {
    #[doc = " Content of the source reference."]
    pub content: String,
    #[doc = " Optional content type (mime type) of the source."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SourceResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: SourceResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " A Stackframe contains the source location."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StackFrame {
    #[doc = " Indicates whether this frame can be restarted with the 'restart' request. Clients should "]
    #[doc = " only use this if the debug adapter supports the 'restart' request (capability "]
    #[doc = " 'supportsRestartRequest' is true)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canRestart")]
    pub can_restart: Option<bool>,
    #[doc = " The column within the line. If source is null or doesn't exist, column is 0 and must be "]
    #[doc = " ignored."]
    pub column: i64,
    #[doc = " An optional end column of the range covered by the stack frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endColumn")]
    pub end_column: Option<i64>,
    #[doc = " An optional end line of the range covered by the stack frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endLine")]
    pub end_line: Option<i64>,
    #[doc = " An identifier for the stack frame. It must be unique across all threads."]
    #[doc = " This id can be used to retrieve the scopes of the frame with the 'scopesRequest' or to "]
    #[doc = " restart the execution of a stackframe."]
    pub id: i64,
    #[doc = " Optional memory reference for the current instruction pointer in this frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructionPointerReference")]
    pub instruction_pointer_reference: Option<String>,
    #[doc = " The line within the file of the frame. If source is null or doesn't exist, line is 0 and "]
    #[doc = " must be ignored."]
    pub line: i64,
    #[doc = " The module associated with this frame, if any."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "moduleId")]
    pub module_id: Option<serde_json::Value>,
    #[doc = " The name of the stack frame, typically a method name."]
    pub name: String,
    #[doc = " An optional hint for how to present this frame in the UI."]
    #[doc = " A value of 'label' can be used to indicate that the frame is an artificial frame that is "]
    #[doc = " used as a visual label or separator. A value of 'subtle' can be used to change the "]
    #[doc = " appearance of a frame in a 'subtle' way."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "presentationHint")]
    pub presentation_hint: Option<String>,
    #[doc = " The optional source of the frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct StackFrameFormat {
    #[doc = " Display the value in hex."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<bool>,
    #[doc = " Includes all stack frames, including those the debug adapter might otherwise hide."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeAll")]
    pub include_all: Option<bool>,
    #[doc = " Displays the line number of the stack frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<bool>,
    #[doc = " Displays the module of the stack frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<bool>,
    #[doc = " Displays the names of parameters for the stack frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameterNames")]
    pub parameter_names: Option<bool>,
    #[doc = " Displays the types of parameters for the stack frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameterTypes")]
    pub parameter_types: Option<bool>,
    #[doc = " Displays the values of parameters for the stack frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameterValues")]
    pub parameter_values: Option<bool>,
    #[doc = " Displays parameters for the stack frame."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<bool>,
}
#[doc = " Arguments for 'stackTrace' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StackTraceArguments {
    #[doc = " Specifies details on how to format the stack frames."]
    #[doc = " The attribute is only honored by a debug adapter if the capability "]
    #[doc = " 'supportsValueFormattingOptions' is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StackFrameFormat>,
    #[doc = " The maximum number of frames to return. If levels is not specified or 0, all frames are "]
    #[doc = " returned."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levels: Option<i64>,
    #[doc = " The index of the first frame to return; if omitted frames start at 0."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startFrame")]
    pub start_frame: Option<i64>,
    #[doc = " Retrieve the stacktrace for this thread."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StackTraceRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: StackTraceArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StackTraceResponseBody {
    #[doc = " The frames of the stackframe. If the array has length zero, there are no stackframes "]
    #[doc = " available."]
    #[doc = " This means that there is no location information available."]
    #[serde(rename = "stackFrames")]
    pub stack_frames: Vec<StackFrame>,
    #[doc = " The total number of frames available in the stack. If omitted or if totalFrames is larger "]
    #[doc = " than the available frames, a client is expected to request frames until a request returns "]
    #[doc = " less frames than requested (which indicates the end of the stack). Returning monotonically "]
    #[doc = " increasing totalFrames values for subsequent requests can be used to enforce paging in the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalFrames")]
    pub total_frames: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StackTraceResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: StackTraceResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'stepBack' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepBackArguments {
    #[doc = " Optional granularity to step. If no granularity is specified, a granularity of 'statement' "]
    #[doc = " is assumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<SteppingGranularity>,
    #[doc = " If this optional flag is true, all other suspended threads are not resumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "singleThread")]
    pub single_thread: Option<bool>,
    #[doc = " Specifies the thread for which to resume execution for one step backwards (of the given "]
    #[doc = " granularity)."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepBackRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: StepBackArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepBackResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'stepIn' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepInArguments {
    #[doc = " Optional granularity to step. If no granularity is specified, a granularity of 'statement' "]
    #[doc = " is assumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<SteppingGranularity>,
    #[doc = " If this optional flag is true, all other suspended threads are not resumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "singleThread")]
    pub single_thread: Option<bool>,
    #[doc = " Optional id of the target to step into."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetId")]
    pub target_id: Option<i64>,
    #[doc = " Specifies the thread for which to resume execution for one step-into (of the given "]
    #[doc = " granularity)."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepInRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: StepInArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepInResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " A StepInTarget can be used in the 'stepIn' request and determines into which single target the "]
#[doc = " stepIn request should step."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepInTarget {
    #[doc = " Unique identifier for a stepIn target."]
    pub id: i64,
    #[doc = " The name of the stepIn target (shown in the UI)."]
    pub label: String,
}
#[doc = " Arguments for 'stepInTargets' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepInTargetsArguments {
    #[doc = " The stack frame for which to retrieve the possible stepIn targets."]
    #[serde(rename = "frameId")]
    pub frame_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepInTargetsRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: StepInTargetsArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepInTargetsResponseBody {
    #[doc = " The possible stepIn targets of the specified source location."]
    pub targets: Vec<StepInTarget>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepInTargetsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: StepInTargetsResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'stepOut' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepOutArguments {
    #[doc = " Optional granularity to step. If no granularity is specified, a granularity of 'statement' "]
    #[doc = " is assumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<SteppingGranularity>,
    #[doc = " If this optional flag is true, all other suspended threads are not resumed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "singleThread")]
    pub single_thread: Option<bool>,
    #[doc = " Specifies the thread for which to resume execution for one step-out (of the given "]
    #[doc = " granularity)."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepOutRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: StepOutArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StepOutResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " The granularity of one 'step' in the stepping requests 'next', 'stepIn', 'stepOut', and "]
#[doc = " 'stepBack'."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum SteppingGranularity {
    #[serde(rename = "statement")]
    Statement,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "instruction")]
    Instruction,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StoppedEventBody {
    #[doc = " If 'allThreadsStopped' is true, a debug adapter can announce that all threads have stopped."]
    #[doc = " - The client should use this information to enable that all threads can be expanded to "]
    #[doc = " access their stacktraces."]
    #[doc = " - If the attribute is missing or false, only the thread with the given threadId can be "]
    #[doc = " expanded."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allThreadsStopped")]
    pub all_threads_stopped: Option<bool>,
    #[doc = " The full reason for the event, e.g. 'Paused on exception'. This string is shown in the UI "]
    #[doc = " as is and must be translated."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = " Ids of the breakpoints that triggered the event. In most cases there will be only a single "]
    #[doc = " breakpoint but here are some examples for multiple breakpoints:"]
    #[doc = " - Different types of breakpoints map to the same location."]
    #[doc = " - Multiple source breakpoints get collapsed to the same instruction by the "]
    #[doc = " compiler/runtime."]
    #[doc = " - Multiple function breakpoints with different function names map to the same location."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hitBreakpointIds")]
    pub hit_breakpoint_ids: Option<Vec<i64>>,
    #[doc = " A value of true hints to the frontend that this event should not change the focus."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preserveFocusHint")]
    pub preserve_focus_hint: Option<bool>,
    #[doc = " The reason for the event."]
    #[doc = " For backward compatibility this string is shown in the UI if the 'description' attribute is "]
    #[doc = " missing (but it must not be translated)."]
    pub reason: String,
    #[doc = " Additional information. E.g. if reason is 'exception', text contains the exception name. "]
    #[doc = " This string is shown in the UI."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = " The thread which was stopped."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threadId")]
    pub thread_id: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StoppedEvent {
    #[doc = " Event-specific information."]
    pub body: StoppedEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'terminate' request."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TerminateArguments {
    #[doc = " A value of true indicates that this 'terminate' request is part of a restart sequence."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TerminateRequest {
    #[doc = " Object containing arguments for the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<TerminateArguments>,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TerminateResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'terminateThreads' request."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TerminateThreadsArguments {
    #[doc = " Ids of threads to be terminated."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threadIds")]
    pub thread_ids: Option<Vec<i64>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TerminateThreadsRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: TerminateThreadsArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TerminateThreadsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TerminatedEventBody {
    #[doc = " A debug adapter may set 'restart' to true (or to an arbitrary object) to request that the "]
    #[doc = " front end restarts the session."]
    #[doc = " The value is not interpreted by the client and passed unmodified as an attribute "]
    #[doc = " '__restart' to the 'launch' and 'attach' requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TerminatedEvent {
    #[doc = " Event-specific information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<TerminatedEventBody>,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " A Thread"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Thread {
    #[doc = " Unique identifier for the thread."]
    pub id: i64,
    #[doc = " A name of the thread."]
    pub name: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ThreadEventBody {
    #[doc = " The reason for the event."]
    pub reason: String,
    #[doc = " The identifier of the thread."]
    #[serde(rename = "threadId")]
    pub thread_id: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ThreadEvent {
    #[doc = " Event-specific information."]
    pub body: ThreadEventBody,
    #[doc = " Type of event."]
    pub event: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ThreadsRequest {
    #[doc = " Object containing arguments for the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ThreadsResponseBody {
    #[doc = " All threads."]
    pub threads: Vec<Thread>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ThreadsResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: ThreadsResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Provides formatting information for a value."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ValueFormat {
    #[doc = " Display the value in hex."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<bool>,
}
#[doc = " A Variable is a name/value pair."]
#[doc = " Optionally a variable can have a 'type' that is shown if space permits or when hovering over "]
#[doc = " the variable's name."]
#[doc = " An optional 'kind' is used to render additional properties of the variable, e.g. different "]
#[doc = " icons can be used to indicate that a variable is public or private."]
#[doc = " If the value is structured (has children), a handle is provided to retrieve the children with "]
#[doc = " the VariablesRequest."]
#[doc = " If the number of named or indexed children is large, the numbers should be returned via the "]
#[doc = " optional 'namedVariables' and 'indexedVariables' attributes."]
#[doc = " The client can use this optional information to present the children in a paged UI and fetch "]
#[doc = " them in chunks."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Variable {
    #[doc = " Optional evaluatable name of this variable which can be passed to the 'EvaluateRequest' to "]
    #[doc = " fetch the variable's value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "evaluateName")]
    pub evaluate_name: Option<String>,
    #[doc = " The number of indexed child variables."]
    #[doc = " The client can use this optional information to present the children in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "indexedVariables")]
    pub indexed_variables: Option<i64>,
    #[doc = " Optional memory reference for the variable if the variable represents executable code, such "]
    #[doc = " as a function pointer."]
    #[doc = " This attribute is only required if the client has passed the value true for the "]
    #[doc = " 'supportsMemoryReferences' capability of the 'initialize' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "memoryReference")]
    pub memory_reference: Option<String>,
    #[doc = " The variable's name."]
    pub name: String,
    #[doc = " The number of named child variables."]
    #[doc = " The client can use this optional information to present the children in a paged UI and "]
    #[doc = " fetch them in chunks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "namedVariables")]
    pub named_variables: Option<i64>,
    #[doc = " Properties of a variable that can be used to determine how to render the variable in the "]
    #[doc = " UI."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "presentationHint")]
    pub presentation_hint: Option<VariablePresentationHint>,
    #[doc = " The type of the variable's value. Typically shown in the UI when hovering over the value."]
    #[doc = " This attribute should only be returned by a debug adapter if the client has passed the "]
    #[doc = " value true for the 'supportsVariableType' capability of the 'initialize' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = " The variable's value."]
    #[doc = " This can be a multi-line text, e.g. for a function the body of a function."]
    #[doc = " For structured variables (which do not have a simple value), it is recommended to provide a "]
    #[doc = " one line representation of the structured object. This helps to identify the structured "]
    #[doc = " object in the collapsed state when its children are not yet visible."]
    #[doc = " An empty string can be used if no value should be shown in the UI."]
    pub value: String,
    #[doc = " If variablesReference is > 0, the variable is structured and its children can be retrieved "]
    #[doc = " by passing variablesReference to the VariablesRequest."]
    #[serde(rename = "variablesReference")]
    pub variables_reference: i64,
}
#[doc = " Optional properties of a variable that can be used to determine how to render the variable in "]
#[doc = " the UI."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct VariablePresentationHint {
    #[doc = " Set of attributes represented as an array of strings. Before introducing additional values, "]
    #[doc = " try to use the listed values."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    #[doc = " The kind of variable. Before introducing additional values, try to use the listed values."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = " If true, clients can present the variable with a UI that supports a specific gesture to "]
    #[doc = " trigger its evaluation."]
    #[doc = " This mechanism can be used for properties that require executing code when retrieving their "]
    #[doc = " value and where the code execution can be expensive and/or produce side-effects. A typical "]
    #[doc = " example are properties based on a getter function."]
    #[doc = " Please note that in addition to the 'lazy' flag, the variable's 'variablesReference' must "]
    #[doc = " refer to a variable that will provide the value through another 'variable' request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy: Option<bool>,
    #[doc = " Visibility of variable. Before introducing additional values, try to use the listed values."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}
#[doc = " Arguments for 'variables' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct VariablesArguments {
    #[doc = " The number of variables to return. If count is missing or 0, all variables are returned."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = " Optional filter to limit the child variables to either named or indexed. If omitted, both "]
    #[doc = " types are fetched."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = " Specifies details on how to format the Variable values."]
    #[doc = " The attribute is only honored by a debug adapter if the capability "]
    #[doc = " 'supportsValueFormattingOptions' is true."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
    #[doc = " The index of the first variable to return; if omitted children start at 0."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[doc = " The Variable reference."]
    #[serde(rename = "variablesReference")]
    pub variables_reference: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct VariablesRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: VariablesArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct VariablesResponseBody {
    #[doc = " All (or a range) of variables for the given variable reference."]
    pub variables: Vec<Variable>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct VariablesResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    pub body: VariablesResponseBody,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " Arguments for 'writeMemory' request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WriteMemoryArguments {
    #[doc = " Optional property to control partial writes. If true, the debug adapter should attempt to "]
    #[doc = " write memory even if the entire memory region is not writable. In such a case the debug "]
    #[doc = " adapter should stop after hitting the first byte of memory that cannot be written and "]
    #[doc = " return the number of bytes written in the response via the 'offset' and 'bytesWritten' "]
    #[doc = " properties."]
    #[doc = " If false or missing, a debug adapter should attempt to verify the region is writable before "]
    #[doc = " writing, and fail the response if it is not."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowPartial")]
    pub allow_partial: Option<bool>,
    #[doc = " Bytes to write, encoded using base64."]
    pub data: String,
    #[doc = " Memory reference to the base location to which data should be written."]
    #[serde(rename = "memoryReference")]
    pub memory_reference: String,
    #[doc = " Optional offset (in bytes) to be applied to the reference location before writing data. Can "]
    #[doc = " be negative."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WriteMemoryRequest {
    #[doc = " Object containing arguments for the command."]
    pub arguments: WriteMemoryArguments,
    #[doc = " The command to execute."]
    pub command: String,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WriteMemoryResponseBody {
    #[doc = " Optional property that should be returned when 'allowPartial' is true to indicate the "]
    #[doc = " number of bytes starting from address that were successfully written."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bytesWritten")]
    pub bytes_written: Option<i64>,
    #[doc = " Optional property that should be returned when 'allowPartial' is true to indicate the "]
    #[doc = " offset of the first byte of data successfully written. Can be negative."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WriteMemoryResponse {
    #[doc = " Contains request result if success is true and optional error details if success is false."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<WriteMemoryResponseBody>,
    #[doc = " The command requested."]
    pub command: String,
    #[doc = " Contains the raw error in short form if 'success' is false."]
    #[doc = " This raw error might be interpreted by the frontend and is not shown in the UI."]
    #[doc = " Some predefined values exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Sequence number of the corresponding request."]
    pub request_seq: i64,
    #[doc = " Sequence number (also known as message ID). For protocol messages of type 'request' this ID "]
    #[doc = " can be used to cancel the request."]
    pub seq: i64,
    #[doc = " Outcome of the request."]
    #[doc = " If true, the request was successful and the 'body' attribute may contain the result of the "]
    #[doc = " request."]
    #[doc = " If the value is false, the attribute 'message' contains the error in short form and the "]
    #[doc = " 'body' may contain additional information (see 'ErrorResponse.body.error')."]
    pub success: bool,
    #[doc = " Message type."]
    #[serde(rename = "type")]
    pub type_: String,
}
pub type Schema = ::std::collections::BTreeMap<String, serde_json::Value>;
