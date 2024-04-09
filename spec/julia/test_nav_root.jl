# Autogenerated from KST: please remove this line if doing any edits by hand!

using Test
import NavRootModule

@testset "NavRoot test" begin
    r = NavRootModule.from_file("src/nav.bin")


    @test r.header.qty_entries == 2
    @test r.header.filename_len == 8
    @test Base.size(r.index.entries, 1) == 2
    @test r.index.entries[1].filename == "FIRST___"
    @test r.index.entries[2].filename == "SECOND__"
end
