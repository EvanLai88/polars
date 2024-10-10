(function() {
    var type_impls = Object.fromEntries([["polars",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-LazySerde%3CT%3E\" class=\"impl\"><a href=\"#impl-Clone-for-LazySerde%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"polars/prelude/enum.LazySerde.html\" title=\"enum polars::prelude::LazySerde\">LazySerde</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"enum\" href=\"polars/prelude/enum.LazySerde.html\" title=\"enum polars::prelude::LazySerde\">LazySerde</a>&lt;T&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.6.0\">1.6.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","polars::prelude::OpaqueColumnUdf"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-LazySerde%3CT%3E\" class=\"impl\"><a href=\"#impl-Debug-for-LazySerde%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"polars/prelude/enum.LazySerde.html\" title=\"enum polars::prelude::LazySerde\">LazySerde</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","polars::prelude::OpaqueColumnUdf"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-LazySerde%3CSpecialEq%3CArc%3Cdyn+ColumnsUdf%3E%3E%3E\" class=\"impl\"><a href=\"#impl-LazySerde%3CSpecialEq%3CArc%3Cdyn+ColumnsUdf%3E%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"enum\" href=\"polars/prelude/enum.LazySerde.html\" title=\"enum polars::prelude::LazySerde\">LazySerde</a>&lt;<a class=\"struct\" href=\"polars/prelude/struct.SpecialEq.html\" title=\"struct polars::prelude::SpecialEq\">SpecialEq</a>&lt;<a class=\"struct\" href=\"polars/prelude/struct.Arc.html\" title=\"struct polars::prelude::Arc\">Arc</a>&lt;dyn <a class=\"trait\" href=\"polars/prelude/trait.ColumnsUdf.html\" title=\"trait polars::prelude::ColumnsUdf\">ColumnsUdf</a>&gt;&gt;&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.materialize\" class=\"method\"><h4 class=\"code-header\">pub fn <a href=\"polars/prelude/enum.LazySerde.html#tymethod.materialize\" class=\"fn\">materialize</a>(self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"polars/prelude/struct.SpecialEq.html\" title=\"struct polars::prelude::SpecialEq\">SpecialEq</a>&lt;<a class=\"struct\" href=\"polars/prelude/struct.Arc.html\" title=\"struct polars::prelude::Arc\">Arc</a>&lt;dyn <a class=\"trait\" href=\"polars/prelude/trait.ColumnsUdf.html\" title=\"trait polars::prelude::ColumnsUdf\">ColumnsUdf</a>&gt;&gt;, <a class=\"enum\" href=\"polars/error/enum.PolarsError.html\" title=\"enum polars::error::PolarsError\">PolarsError</a>&gt;</h4></section></div></details>",0,"polars::prelude::OpaqueColumnUdf"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-LazySerde%3CT%3E\" class=\"impl\"><a href=\"#impl-PartialEq-for-LazySerde%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"enum\" href=\"polars/prelude/enum.LazySerde.html\" title=\"enum polars::prelude::LazySerde\">LazySerde</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"enum\" href=\"polars/prelude/enum.LazySerde.html\" title=\"enum polars::prelude::LazySerde\">LazySerde</a>&lt;T&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.6.0\">1.6.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#261\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","polars::prelude::OpaqueColumnUdf"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[8377]}