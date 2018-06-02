defmodule RustlerTest.PrimitivesTest do
  use ExUnit.Case, async: true

  test "datetime decoding and encoding" do
    datetime = DateTime.utc_now()
    assert datetime == RustlerTest.datetime_echo(datetime)
  end

  test "doesn't accept an invalid datetime" do
    assert_raise ArgumentError, fn -> RustlerTest.datetime_echo(123) end
  end
end
