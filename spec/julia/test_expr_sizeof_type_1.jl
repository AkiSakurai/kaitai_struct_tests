# Autogenerated from KST: please remove this line if doing any edits by hand!

using Test
import ExprSizeofType1Module

@testset "ExprSizeofType1 test" begin
    r = ExprSizeofType1Module.from_file("src/fixed_struct.bin")


    @test r.sizeof_block == ((1 + 4) + 2) + 4
    @test r.sizeof_subblock == 4
end
