// <auto-generated/> by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using DiplomatFeatures.Diplomat;
#pragma warning restore 0105

namespace DiplomatFeatures;

#nullable enable

public partial class OpaqueMutexedString: IDisposable
{
    private unsafe Raw.OpaqueMutexedString* _inner;

    /// <summary>
    /// Creates a managed <c>OpaqueMutexedString</c> from a raw handle.
    /// </summary>
    /// <remarks>
    /// Safety: you should not build two managed objects using the same raw handle (may causes use-after-free and double-free).
    /// <br/>
    /// This constructor assumes the raw struct is allocated on Rust side.
    /// If implemented, the custom Drop implementation on Rust side WILL run on destruction.
    /// </remarks>
    public unsafe OpaqueMutexedString(Raw.OpaqueMutexedString* handle)
    {
        _inner = handle;
    }

    /// <returns>
    /// A <c>OpaqueMutexedString</c> allocated on Rust side.
    /// </returns>
    public static OpaqueMutexedString FromUsize(nuint number)
    {
        unsafe
        {
            Raw.OpaqueMutexedString* retVal = Raw.OpaqueMutexedString.FromUsize(number);
            return new OpaqueMutexedString(retVal);
        }
    }

    public void Change(nuint number)
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("OpaqueMutexedString");
            }
            Raw.OpaqueMutexedString.Change(_inner, number);
        }
    }

    public nuint GetLenAndAdd(nuint other)
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("OpaqueMutexedString");
            }
            nuint retVal = Raw.OpaqueMutexedString.GetLenAndAdd(_inner, other);
            return retVal;
        }
    }

    public string DummyStr()
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("OpaqueMutexedString");
            }
            Raw.string retVal = Raw.OpaqueMutexedString.DummyStr(_inner);
            return expected named type name, found `&'a DiplomatStr`;
        }
    }

    /// <returns>
    /// A <c>Utf16Wrap</c> allocated on Rust side.
    /// </returns>
    public Utf16Wrap Wrapper()
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("OpaqueMutexedString");
            }
            Raw.Utf16Wrap* retVal = Raw.OpaqueMutexedString.Wrapper(_inner);
            return new Utf16Wrap(retVal);
        }
    }

    /// <summary>
    /// Returns the underlying raw handle.
    /// </summary>
    public unsafe Raw.OpaqueMutexedString* AsFFI()
    {
        return _inner;
    }

    /// <summary>
    /// Destroys the underlying object immediately.
    /// </summary>
    public void Dispose()
    {
        unsafe
        {
            if (_inner == null)
            {
                return;
            }

            Raw.OpaqueMutexedString.Destroy(_inner);
            _inner = null;

            GC.SuppressFinalize(this);
        }
    }

    ~OpaqueMutexedString()
    {
        Dispose();
    }
}