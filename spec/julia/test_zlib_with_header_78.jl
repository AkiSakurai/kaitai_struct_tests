# Autogenerated from KST: please remove this line if doing any edits by hand!

using Test
import ZlibWithHeader78Module

@testset "ZlibWithHeader78 test" begin
    r = ZlibWithHeader78Module.from_file("src/zlib_with_header_78.bin")


    @test r.data == Vector{UInt8}([97, 32, 113, 117, 105, 99, 107, 32, 98, 114, 111, 119, 110, 32, 102, 111, 120, 32, 106, 117, 109, 112, 115, 32, 111, 118, 101, 114])
end
