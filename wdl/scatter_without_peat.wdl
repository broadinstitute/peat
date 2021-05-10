version 1.0

workflow echo {
    input {
        Int n_jobs
        String out_file_name
    }
    String worker_out_file_name_base = "worker_out"
    scatter(i_job in range(n_jobs)) {
        String worker_out_file_name = worker_out_file_name_base + "." + i_job + ".txt"
        call worker {
            input:
                i_job = i_job,
                out_file_name = worker_out_file_name
        }
    }
    call reducer {
        input:
            in_files = worker.out_file,
            out_file_name = out_file_name
    }
}

task worker {
    input {
        Int i_job
        String out_file_name
    }
    runtime {
        docker: "ubuntu:21.04"
    }
    command <<<
        echo "Hello, world, this is job ~{i_job}!" > ~{out_file_name}
    >>>
    output {
        File out_file = out_file_name
    }
}

task reducer {
    input {
        Array[File] in_files
        String out_file_name
    }
    runtime {
        docker: "ubuntu:21.04"
    }
    command <<<
        cat ~{sep=' ' in_files} > ~{out_file_name}
    >>>
    output {
        File out_file = out_file_name
    }
}