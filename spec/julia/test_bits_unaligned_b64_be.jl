# Autogenerated from KST: please remove this line if doing any edits by hand!

using Test
import BitsUnalignedB64BeModule

@testset "BitsUnalignedB64Be test" begin
    r = BitsUnalignedB64BeModule.from_file("src/process_xor_4.bin")


    @test r.a == true
    @test r.b == 15670070570729969769
    @test r.c == 14
end
