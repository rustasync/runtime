(function() {var implementors = {};
implementors["romio"] = [{text:"impl <a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"romio/struct.TcpStream.html\" title=\"struct romio::TcpStream\">TcpStream</a>",synthetic:false,types:["romio::tcp::stream::TcpStream"]},{text:"impl <a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"romio/uds/struct.UnixStream.html\" title=\"struct romio::uds::UnixStream\">UnixStream</a>",synthetic:false,types:["romio::uds::stream::UnixStream"]},{text:"impl&lt;E&gt; <a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"romio/raw/struct.PollEvented.html\" title=\"struct romio::raw::PollEvented\">PollEvented</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"mio/event_imp/trait.Evented.html\" title=\"trait mio::event_imp::Evented\">Evented</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,&nbsp;</span>",synthetic:false,types:["romio::raw::poll_evented::PollEvented"]},];
implementors["runtime"] = [{text:"impl <a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"runtime/net/tcp/struct.TcpStream.html\" title=\"struct runtime::net::tcp::TcpStream\">TcpStream</a>",synthetic:false,types:["runtime::net::tcp::TcpStream"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
