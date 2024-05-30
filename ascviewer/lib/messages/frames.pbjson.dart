//
//  Generated code. Do not modify.
//  source: frames.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use framesReqDescriptor instead')
const FramesReq$json = {
  '1': 'FramesReq',
};

/// Descriptor for `FramesReq`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List framesReqDescriptor = $convert.base64Decode(
    'CglGcmFtZXNSZXE=');

@$core.Deprecated('Use signalDescriptor instead')
const Signal$json = {
  '1': 'Signal',
  '2': [
    {'1': 'sig_name', '3': 1, '4': 1, '5': 9, '10': 'sigName'},
    {'1': 'sig_value', '3': 2, '4': 1, '5': 9, '10': 'sigValue'},
  ],
};

/// Descriptor for `Signal`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signalDescriptor = $convert.base64Decode(
    'CgZTaWduYWwSGQoIc2lnX25hbWUYASABKAlSB3NpZ05hbWUSGwoJc2lnX3ZhbHVlGAIgASgJUg'
    'hzaWdWYWx1ZQ==');

@$core.Deprecated('Use frameItemDescriptor instead')
const FrameItem$json = {
  '1': 'FrameItem',
  '2': [
    {'1': 'msg_id', '3': 1, '4': 1, '5': 5, '10': 'msgId'},
    {'1': 'msg_name', '3': 2, '4': 1, '5': 9, '10': 'msgName'},
    {'1': 'direction', '3': 3, '4': 1, '5': 9, '10': 'direction'},
    {'1': 'signals', '3': 4, '4': 3, '5': 11, '6': '.frames.Signal', '10': 'signals'},
  ],
};

/// Descriptor for `FrameItem`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List frameItemDescriptor = $convert.base64Decode(
    'CglGcmFtZUl0ZW0SFQoGbXNnX2lkGAEgASgFUgVtc2dJZBIZCghtc2dfbmFtZRgCIAEoCVIHbX'
    'NnTmFtZRIcCglkaXJlY3Rpb24YAyABKAlSCWRpcmVjdGlvbhIoCgdzaWduYWxzGAQgAygLMg4u'
    'ZnJhbWVzLlNpZ25hbFIHc2lnbmFscw==');

@$core.Deprecated('Use framesRespDescriptor instead')
const FramesResp$json = {
  '1': 'FramesResp',
  '2': [
    {'1': 'frames', '3': 1, '4': 3, '5': 11, '6': '.frames.FrameItem', '10': 'frames'},
  ],
};

/// Descriptor for `FramesResp`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List framesRespDescriptor = $convert.base64Decode(
    'CgpGcmFtZXNSZXNwEikKBmZyYW1lcxgBIAMoCzIRLmZyYW1lcy5GcmFtZUl0ZW1SBmZyYW1lcw'
    '==');

