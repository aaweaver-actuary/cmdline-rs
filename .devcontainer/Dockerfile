FROM alpine:3.19

WORKDIR /cmdline-rs

RUN apk add --no-cache \
        zsh \
        curl \
        git \
        make \
        openssh \
        bash \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && source $HOME/.cargo/env \
    && cargo install cargo-edit \
    && cargo install cargo-watch \
    && cargo install cargo-audit \
    && rustup component add \
        rustfmt \
        clippy \
    && apk del curl \
    && git clone http://github.com/aaweaver-actuary/dotfiles \
    && cp ./dotfiles/install_dotfiles /usr/bin/install_dotfiles \
    && chmod +x /usr/bin/install_dotfiles \
    && rm -rf ./dotfiles \
    && install_dotfiles ~ install_oh_my_zsh \
    && chmod +x ~/install_oh_my_zsh \
    && ~/install_oh_my_zsh \
    && rm -rf ~/install_oh_my_zsh \
    && install_dotfiles ~ .zshrc .zsh_aliases \
    && touch ~/.passwords \    
    && rm -rf /var/cache/apk/* \
    && rm -rf /tmp/*

CMD ["/bin/zsh"]