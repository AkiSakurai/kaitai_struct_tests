# Autogenerated from KST: please remove this line if doing any edits by hand!

using Test
import ProcessXorConstModule

@testset "ProcessXorConst test" begin
    r = ProcessXorConstModule.from_file("src/process_xor_1.bin")


    @test r.key == 255
    @test r.buf == Vector{UInt8}([102, 111, 111, 32, 98, 97, 114])
end
