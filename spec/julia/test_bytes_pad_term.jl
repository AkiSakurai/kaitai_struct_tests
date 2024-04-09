# Autogenerated from KST: please remove this line if doing any edits by hand!

using Test
import BytesPadTermModule

@testset "BytesPadTerm test" begin
    r = BytesPadTermModule.from_file("src/str_pad_term.bin")


    @test r.str_pad == Vector{UInt8}([115, 116, 114, 49])
    @test r.str_term == Vector{UInt8}([115, 116, 114, 50, 102, 111, 111])
    @test r.str_term_and_pad == Vector{UInt8}([115, 116, 114, 43, 43, 43, 51, 98, 97, 114, 43, 43, 43])
    @test r.str_term_include == Vector{UInt8}([115, 116, 114, 52, 98, 97, 122, 64])
end
