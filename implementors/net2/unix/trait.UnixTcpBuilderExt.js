(function() {var implementors = {};
implementors["hyper"] = [];
implementors["mio"] = [];
implementors["net2"] = [];
implementors["reqwest"] = [];
implementors["tokio_core"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()