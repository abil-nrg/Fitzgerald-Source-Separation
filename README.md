# Fitzgerald-Source-Separation

## Overview

An implementation of an interactive audio source seperation system based on **Derry Fitzgerald's median filtering algorithm** [1]. We take a polyphonic audio file and decompose it into its harmonic (tonal instruments) and percussive (drums) components.

Rust is used for the algorithm and GUI.

## Getting Started

Once project is finished describe how to run it. Include some pictures here as well.

## Introduction

Our problem is Music Source Seperation, the decomposition of polyphonic audio into it's individual components. MSS has been used effectively in a variety of disciplines, such as manatee population estimation [2], cardiac feature monitoring [3], and bird sound labelling [4]. It has applications in music production and vocal removal for karaoke.

We will be implementing the algorithm described by Derry FitzGerald in his paper "Harmonic/Percussive Seperation using Median Filtering" [1]. A non-deep learning, deterministic appraoch to source seperation. The result will be visualized in a GUI impleneted in Rust using custom widgets and **egui** [5]. The extensive monomorphization of generic code and "const generics" performed by the `rustc` compiler allows for extremely high-performance code generation, while preserving compile-time guarantees of memory safety [6].


## The Algorithm  
Let $f$ be the frequency and $t$ time.  
1. Given an input audio signal apply **Short Time Fourier Transform** (STFT) to get $S(f,t)$, the magnitude of a frequence $f$ at time $t$.  
2. Define the **median filter** as 
```math
y(n) = \text{median} \{ x(n - k : x + k): k = (l-2)/2\}
```
For some $x(n)$, an array, and $l$, a filter size.  
3.  Let $P(t,f)$ be the median filter of $S(\cdot, f)$  
4.  Let $H(t,f)$ be the median filter of $S(t, \cdot)$  
5. Define our **binary filters** [1] as  
```math
    M_H (t, f) = \begin{cases}
        1,& H(t, f) > P(t,f)\\
        0,& \text{otherwise}
    \end{cases}\\
```
```math
    M_P (t, f) = \begin{cases}
        1,& P(t, f) > H(t,f)\\
        0,& \text{otherwise}
    \end{cases}
```  
6. Apply the **Inverse Short Time Fourier Transform** (iSTFT) to return the masked spectograms to the time domain. 

## Implementation

Our implementation will be guided by the section on Harmonic–Percussive Separation in the *Fundamentals of Music Processing* notebooks [7].

The algorithm as described will be implemented in Rust as a library crate. This will include an implementation of the Short Time Fourier Transform (STFT) using the Fast Fourier Transform, a variety of window functions, the median filter, and the algorithm itself. As memory safety will be a priority in this part of the implementation, we will not use any `unsafe` code. The code will be as generic as possible, using the traits provided by the `num-traits` crate [8] as trait bounds. The implementation will have a high level of unit testing, using the `cargo test` testing harness.

In a separate binary crate, we will design and implement a visualization of the algorithm pipeline in **egui** [5] using custom widgets. This will depend upon the crate using the implementation of the algorithm. The user will be prompted to open an audio file, then they will be able to tweak the algorithm parameters, such as the median filter length or window function. Each step of the algorithm will be displayed as a spectrogram in the application, visualizing how the parameters change the output. A benefit of using **egui** as the basis for our implementation is that can target both the native system and a WebAssembly (WASM) target. If we design it with WASM compatibility in mind, we will be able to publish our final application as a public website.

## References

[1] D. FitzGerald, "Harmonic/Percussive Separation using Median Filtering," in *13th International Conference on Digital Audio Effects (DAFx-10)*, Graz, Austria, 2010.

[2] M. C. G. Hillis, et al., "Manatee population estimation via automated acoustic monitoring," *Journal of the Acoustical Society of America*, 2021.

[3] R. Onu, et al., "Cardiac feature monitoring using complementary diffusion," *IEEE Transactions on Biomedical Engineering*, 2014.

[4] T. Poutaraud, et al., "Bird sound labelling using source separation techniques," *Applied Acoustics*, vol. 216, 2024.

[5] E. Ernerfeldt, "egui: An easy-to-use immediate mode GUI in Rust that runs on both web and native," 2024. [Online]. Available: [https://github.com/emilk/egui](https://github.com/emilk/egui)

[6] Rust Project Developers, "The Rustc Book: Monomorphization and Const Generics," 2024. [Online]. Available: [https://doc.rust-lang.org/rustc/](https://doc.rust-lang.org/rustc/)

[7] M. Müller, *Fundamentals of Music Processing: Audio, Analysis, Algorithms, Applications*. Springer, 2015.

[8] P. Stone, et al., "num-traits: Numeric traits for Rust," 2024. [Online]. Available: [https://github.com/rust-num/num-traits](https://github.com/rust-num/num-traits)

