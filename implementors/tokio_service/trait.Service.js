(function() {var implementors = {};
implementors["hyper_tls"] = ["impl&lt;T:&nbsp;<a class=\"trait\" href=\"hyper/client/connect/trait.Connect.html\" title=\"trait hyper::client::connect::Connect\">Connect</a>&gt; <a class=\"trait\" href=\"tokio_service/trait.Service.html\" title=\"trait tokio_service::Service\">Service</a> for <a class=\"struct\" href=\"hyper_tls/struct.HttpsConnector.html\" title=\"struct hyper_tls::HttpsConnector\">HttpsConnector</a>&lt;T&gt;",];
implementors["reqwest"] = ["impl&lt;R, S, E&gt; <a class=\"trait\" href=\"tokio_service/trait.Service.html\" title=\"trait tokio_service::Service\">Service</a> for <a class=\"struct\" href=\"tokio_proto/util/client_proxy/struct.ClientProxy.html\" title=\"struct tokio_proto::util::client_proxy::ClientProxy\">ClientProxy</a>&lt;R, S, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;,&nbsp;</span>","impl <a class=\"trait\" href=\"tokio_service/trait.Service.html\" title=\"trait tokio_service::Service\">Service</a> for <a class=\"struct\" href=\"hyper/client/connect/struct.HttpConnector.html\" title=\"struct hyper::client::connect::HttpConnector\">HttpConnector</a>","impl&lt;C, B&gt; <a class=\"trait\" href=\"tokio_service/trait.Service.html\" title=\"trait tokio_service::Service\">Service</a> for <a class=\"struct\" href=\"hyper/client/struct.Client.html\" title=\"struct hyper::client::Client\">Client</a>&lt;C, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a>&lt;Error = <a class=\"enum\" href=\"hyper/error/enum.Error.html\" title=\"enum hyper::error::Error\">Error</a>&gt; + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"hyper/client/connect/trait.Connect.html\" title=\"trait hyper::client::connect::Connect\">Connect</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;B as <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a>&gt;::<a class=\"type\" href=\"futures/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures::stream::Stream::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;,&nbsp;</span>","impl&lt;T&gt; <a class=\"trait\" href=\"tokio_service/trait.Service.html\" title=\"trait tokio_service::Service\">Service</a> for <a class=\"struct\" href=\"hyper_tls/client/struct.HttpsConnector.html\" title=\"struct hyper_tls::client::HttpsConnector\">HttpsConnector</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"hyper/client/connect/trait.Connect.html\" title=\"trait hyper::client::connect::Connect\">Connect</a>,&nbsp;</span>",];
implementors["tokio_proto"] = ["impl&lt;T, P&gt; <a class=\"trait\" href=\"tokio_service/trait.Service.html\" title=\"trait tokio_service::Service\">Service</a> for <a class=\"struct\" href=\"tokio_proto/pipeline/struct.ClientService.html\" title=\"struct tokio_proto::pipeline::ClientService\">ClientService</a>&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"tokio_proto/pipeline/trait.ClientProto.html\" title=\"trait tokio_proto::pipeline::ClientProto\">ClientProto</a>&lt;T&gt;,&nbsp;</span>","impl&lt;T, P&gt; <a class=\"trait\" href=\"tokio_service/trait.Service.html\" title=\"trait tokio_service::Service\">Service</a> for <a class=\"struct\" href=\"tokio_proto/multiplex/struct.ClientService.html\" title=\"struct tokio_proto::multiplex::ClientService\">ClientService</a>&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"tokio_proto/multiplex/trait.ClientProto.html\" title=\"trait tokio_proto::multiplex::ClientProto\">ClientProto</a>&lt;T&gt;,&nbsp;</span>","impl&lt;R, S, E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;&gt; <a class=\"trait\" href=\"tokio_service/trait.Service.html\" title=\"trait tokio_service::Service\">Service</a> for <a class=\"struct\" href=\"tokio_proto/util/client_proxy/struct.ClientProxy.html\" title=\"struct tokio_proto::util::client_proxy::ClientProxy\">ClientProxy</a>&lt;R, S, E&gt;",];
implementors["tokio_service"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
