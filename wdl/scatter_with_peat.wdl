version 1.0

workflow echo {
    input {
        Int n_jobs
        Int n_groups
        String out_file_name
    }
    String worker_out_file_prefix = "worker_out."
    String worker_out_file_suffix = ".txt"
    scatter(i_group in range(n_groups)) {
        call worker {
            input:
                n_jobs = n_jobs,
                n_groups = n_groups,
                i_group = i_group,
                out_file_prefix = worker_out_file_prefix,
                out_file_suffix = worker_out_file_suffix,
        }
    }
    call reducer {
        input:
            in_files = flatten(worker.out_files),
            out_file_name = out_file_name
    }
}

task worker {
    input {
        Int n_jobs
        Int n_groups
        Int i_group
        String out_file_prefix
        String out_file_suffix
    }
    runtime {
        docker: "gcr.io/nitrogenase-docker/peat:1.0.0-ubuntu"
    }
    command <<<
        peat << EOF
        Peat 1.0
        N_JOBS = ~{n_jobs}
        N_GROUPS = ~{n_groups}
        I_GROUP = ~{i_group}
        I <- 0 .. N_JOBS / 0 .. N_GROUPS $ I_GROUP
        ===
        echo "Hello, world, this is job <:I:> from group <:I_GROUP:>!" \
              > ~{out_file_prefix}<:I:>~{out_file_suffix}
        EOF
    >>>
    output {
        Array[File] out_files = glob(out_file_prefix + "*" + out_file_suffix)
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