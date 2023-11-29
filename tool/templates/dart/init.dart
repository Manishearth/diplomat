
/// A [Rune] is a Unicode code point, such as `a`, or `💡`.
/// 
/// The recommended way to obtain a [Rune] is to create it from a 
/// [String], which is conceptually a list of [Runes]. For example,
/// `'a'.runes.first` is equal to the [Rune] `a`.
/// 
/// Dart does not have a character/rune literal, so integer literals
/// need to be used. For example the Unicode code point U+1F4A1, `💡`,
/// can be represented by `0x1F4A1`. Note that only values in the ranges
/// `0x0..0xD7FF` and `0xE000..0x10FFFF` (both inclusive) are Unicode
/// code points, and hence valid [Rune]s.
///
/// A [String] can be constructed from a [Rune] using [String.fromCharCode]. 
typedef Rune = int;
/// A list of [Rune]s.
typedef RuneList = Uint32List;

late final ffi.Pointer<T> Function<T extends ffi.NativeType>(String) _capi;
void init(String path) => _capi = ffi.DynamicLibrary.open(path).lookup;

// ignore: unused_element
final _callocFree = core.Finalizer(ffi2.calloc.free);

// ignore: unused_element
extension _UtfViews on String {
  _Utf8View get utf8View => _Utf8View(this);
  _Utf16View get utf16View => _Utf16View(this);
}

/// An unspecified error value
// ignore: unused_element
class VoidError {
  @override
  bool operator ==(Object other) => other is VoidError;

  @override
  int get hashCode => 1;
}

// ignore: unused_element
class _Utf8View {
  final Uint8List _codeUnits;

  // Copies
  _Utf8View(String string) : this._codeUnits = Utf8Encoder().convert(string);

  ffi.Pointer<ffi.Uint8> pointer(ffi.Allocator alloc) {
    // Copies
    return alloc<ffi.Uint8>(length)..asTypedList(length).setAll(0, _codeUnits);
  }

  int get length => _codeUnits.length;
}

// ignore: unused_element
class _Utf16View {
  final core.List<int> _codeUnits;

  _Utf16View(String string) : this._codeUnits = string.codeUnits;

  ffi.Pointer<ffi.Uint16> pointer(ffi.Allocator alloc) {
    // Copies
    return alloc<ffi.Uint16>(length)..asTypedList(length).setAll(0, _codeUnits);
  }

  int get length => _codeUnits.length;
}

// ignore: unused_element
class _SizeListView {
  final core.List<int> _values;

  _SizeListView(this._values);

  // Copies
  ffi.Pointer<ffi.Size> pointer(ffi.Allocator alloc) {
    final pointer = alloc<ffi.Size>(_values.length);
    for (var i = 0; i < _values.length; i++) {
      pointer[i] = _values[i];
    }
    return pointer;
  }

  int get length => _values.length;
}

// ignore: unused_element
extension _Int8ListFfi on Int8List {
  ffi.Pointer<ffi.Int8> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Int8>(length)..asTypedList(length).setAll(0, this);
  }
}

// ignore: unused_element
extension _Int16ListFfi on Int16List {
  ffi.Pointer<ffi.Int16> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Int16>(length)..asTypedList(length).setAll(0, this);
  }
}

// ignore: unused_element
extension _Int32ListFfi on Int32List {
  ffi.Pointer<ffi.Int32> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Int32>(length)..asTypedList(length).setAll(0, this);
  }
}

// ignore: unused_element
extension _Int64ListFfi on Int64List {
  ffi.Pointer<ffi.Int64> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Int64>(length)..asTypedList(length).setAll(0, this);
  }
}

// ignore: unused_element
extension _Uint8ListFfi on Uint8List {
  ffi.Pointer<ffi.Uint8> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Uint8>(length)..asTypedList(length).setAll(0, this);
  }
}

// ignore: unused_element
extension _Uint16ListFfi on Uint16List {
  ffi.Pointer<ffi.Uint16> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Uint16>(length)..asTypedList(length).setAll(0, this);
  }
}

// ignore: unused_element
extension _Uint32ListFfi on Uint32List {
  ffi.Pointer<ffi.Uint32> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Uint32>(length)..asTypedList(length).setAll(0, this);
  }
}

// ignore: unused_element
extension _Uint64ListFfi on Uint64List {
  ffi.Pointer<ffi.Uint64> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Uint64>(length)..asTypedList(length).setAll(0, this);
  }
}

// ignore: unused_element
extension _Float32ListFfi on Float32List {
  ffi.Pointer<ffi.Float> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Float>(length)..asTypedList(length).setAll(0, this);
  }
}

// ignore: unused_element
extension _Float64ListFfi on Float64List {
  ffi.Pointer<ffi.Double> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Double>(length)..asTypedList(length).setAll(0, this);
  }
}