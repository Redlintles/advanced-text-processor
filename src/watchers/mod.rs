use std::{collections::HashMap, fs::OpenOptions, io::Write, path::Path};

use crate::{
    utils::errors::{TextForgeError, TextForgeErrorCode},
    watchers::ExecutionWindows::All,
};
use std::borrow::Cow;

pub mod default_watchers;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct WatcherContext {
    pub current: Arc<str>,
    pub before: Arc<str>,
    pub after: Option<Arc<str>>,
    pub instruction: Arc<str>,
}

impl WatcherContext {
    /// Constrói um `WatcherContext` a partir de seus quatro campos.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textforge::watchers::WatcherContext;
    /// use std::sync::Arc;
    ///
    /// let ctx = WatcherContext::new(
    ///     "banana!",
    ///     "banana",
    ///     Some("BANANA!"),
    ///     "add_to_end",
    /// );
    ///
    /// assert_eq!(ctx.current.as_ref(), "banana!");
    /// assert_eq!(ctx.after.as_deref(), Some("BANANA!"));
    /// ```

    pub fn new<C, B, A, I>(current: C, before: B, after: Option<A>, instruction: I) -> Self
    where
        C: Into<Arc<str>>,
        B: Into<Arc<str>>,
        A: Into<Arc<str>>,
        I: Into<Arc<str>>,
    {
        WatcherContext {
            current: current.into(),
            before: before.into(),
            after: after.map(Into::into),
            instruction: instruction.into(),
        }
    }
}

/// Decide quando os watchers de uma [`WatcherList`] são executados em
/// relação às instruções do pipeline.
///
/// Atualmente só `All` (todo passo) está implementado; as demais
/// variantes planejadas ficam comentadas até terem uma execução real
/// em [`WatcherList::run_watchers`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionWindows {
    /// O watcher roda em toda instrução do pipeline.
    All = 0,
    // Every(usize),
    // After(usize),
    // Before(usize),
}

/// Registro de watchers e agenda de execução para um pipeline ATP.
///
/// Um `WatcherList` guarda três coisas separadas:
/// - `watchers`: funções nomeadas que recebem um [`WatcherContext`] e
///   retornam uma `String` (via [`set_watcher`](WatcherList::set_watcher));
/// - `schedule`: em quais momentos cada watcher registrado deve rodar
///   (via [`schedule_watcher`](WatcherList::schedule_watcher));
/// - `result`: o relatório acumulado, indexado por número do passo do
///   pipeline (populado por [`run_watchers`](WatcherList::run_watchers),
///   exportável via [`to_json`](WatcherList::to_json)).
///
/// `watchers` e `schedule` sobrevivem a [`reset`](WatcherList::reset) —
/// só `result` e `counter` são limpos — pra permitir reusar a mesma
/// instância em múltiplas execuções do pipeline sem reregistrar tudo.
///
/// # Examples
///
/// ```rust
/// use textforge::watchers::{WatcherList, ExecutionWindows, WatcherContext};
///
/// let mut watchers = WatcherList::default();
///
/// watchers.set_watcher("current_len", |ctx: WatcherContext| {
///     ctx.current.len().to_string()
/// });
/// watchers.schedule_watcher("current_len", ExecutionWindows::All)?;
///
/// let ctx = WatcherContext::new(
///     "banana".to_string(),
///     "".to_string(),
///     None::<&str>,
///     "add_to_end".to_string(),
/// );
/// watchers.run_watchers(ctx)?;
/// # Ok::<(), textforge::utils::errors::TextForgeError>(())
/// ```
#[derive(Default)]
pub struct WatcherList {
    result: Vec<HashMap<String, String>>,
    watchers: HashMap<&'static str, Box<dyn (Fn(WatcherContext) -> String) + 'static>>,
    schedule: HashMap<&'static str, ExecutionWindows>,
    counter: u64,
}

impl WatcherList {
    /// Cria uma `WatcherList` vazia. Alias de [`Default::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Limpa o relatório acumulado (`result`) e zera o contador de passos.
    ///
    /// Watchers registrados e o agendamento (`watchers`/`schedule`) **não**
    /// são afetados — a instância continua pronta pra rodar outro pipeline
    /// sem precisar reregistrar nada.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textforge::watchers::{WatcherList, ExecutionWindows, WatcherContext};
    ///
    /// let mut watchers = WatcherList::default();
    /// watchers.set_watcher("len", |ctx: WatcherContext| ctx.current.len().to_string());
    /// watchers.schedule_watcher("len", ExecutionWindows::All)?;
    ///
    /// let ctx = WatcherContext::new("ab", "", None::<&str>, "noop");
    /// watchers.run_watchers(ctx)?;
    ///
    /// watchers.reset();
    /// // "len" continua registrado e agendado após o reset.
    /// let ctx2 = WatcherContext::new("abc", "", None::<&str>, "noop");
    /// watchers.run_watchers(ctx2)?;
    /// # Ok::<(), textforge::utils::errors::TextForgeError>(())
    /// ```
    pub fn reset(&mut self) -> () {
        self.counter = 0;
        self.result = Vec::new();
    }

    fn add_to_result(
        &mut self,
        counter: u64,
        watcher_name: String,
        return_value: String,
    ) -> Result<(), TextForgeError> {
        self.result.push(HashMap::new());
        let iteration_result = self.result.get_mut(counter as usize).ok_or_else(|| {
            TextForgeError::new(
                TextForgeErrorCode::GenericError(Cow::from("Iteration result not found")),
                Cow::from("add_to_result"),
                Cow::from("vec.get"),
            )
        })?;

        iteration_result.insert(watcher_name, return_value);

        Ok(())
    }

    /// Registra (ou substitui) um watcher sob `watcher_name`.
    ///
    /// Registrar um watcher **não** o agenda pra rodar — é preciso chamar
    /// [`schedule_watcher`](WatcherList::schedule_watcher) separadamente.
    /// Chamar `set_watcher` de novo com um nome já usado substitui a
    /// função anterior silenciosamente; o agendamento existente pra esse
    /// nome (se houver) não é afetado.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textforge::watchers::{WatcherList, WatcherContext};
    ///
    /// let mut watchers = WatcherList::default();
    /// watchers.set_watcher("is_empty", |ctx: WatcherContext| ctx.current.is_empty().to_string());
    /// ```
    pub fn set_watcher<F>(&mut self, watcher_name: &'static str, watcher: F)
    where
        F: Fn(WatcherContext) -> String + 'static,
    {
        self.watchers.insert(watcher_name, Box::new(watcher));
    }

    /// Agenda um watcher já registrado pra rodar segundo `when`.
    ///
    /// # Errors
    ///
    /// Retorna `Err` se `watcher_name` não corresponder a nenhum watcher
    /// registrado via [`set_watcher`](WatcherList::set_watcher).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textforge::watchers::{WatcherList, ExecutionWindows, WatcherContext};
    ///
    /// let mut watchers = WatcherList::default();
    /// watchers.set_watcher("len", |ctx: WatcherContext| ctx.current.len().to_string());
    ///
    /// assert!(watchers.schedule_watcher("len", ExecutionWindows::All).is_ok());
    /// assert!(watchers.schedule_watcher("nao_existe", ExecutionWindows::All).is_err());
    /// ```
    pub fn schedule_watcher(
        &mut self,
        watcher_name: &'static str,
        when: ExecutionWindows,
    ) -> Result<(), TextForgeError> {
        if self.watchers.contains_key(watcher_name) {
            self.schedule.insert(watcher_name, when);
            return Ok(());
        }
        return Err(TextForgeError::new(
            crate::utils::errors::TextForgeErrorCode::WatcherNotFoundError(Cow::from(format!(
                "Watcher {} not found",
                watcher_name
            ))),
            Cow::from("WatcherList.schedule_watcher"),
            Cow::from(watcher_name.to_string()),
        ));
    }

    /// Executa todos os watchers agendados contra `input`, armazenando
    /// cada resultado sob o passo atual (`counter`) no relatório interno,
    /// e então incrementa `counter`.
    ///
    /// Pensado pra ser chamado uma vez por instrução dentro de um loop de
    /// processamento (ver `TextForgeProcessor::process_all_with_watchers`), na
    /// mesma ordem em que as instruções do pipeline rodam — a correção do
    /// relatório depende dessa ordem, já que `counter` não é derivado de
    /// `input`, só da sequência de chamadas.
    ///
    /// # Errors
    ///
    /// Propaga qualquer erro do watcher subjacente (hoje, apenas o caso
    /// interno de inconsistência entre `schedule` e `watchers`, que não é
    /// alcançável através da API pública atual).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textforge::watchers::{WatcherList, ExecutionWindows, WatcherContext};
    ///
    /// let mut watchers = WatcherList::default();
    /// watchers.set_watcher("len", |ctx: WatcherContext| ctx.current.len().to_string());
    /// watchers.schedule_watcher("len", ExecutionWindows::All)?;
    ///
    /// let ctx = WatcherContext::new("banana", "", None::<&str>, "add_to_end");
    /// watchers.run_watchers(ctx)?;
    /// # Ok::<(), textforge::utils::errors::TextForgeError>(())
    /// ```
    pub fn run_watchers(&mut self, input: WatcherContext) -> Result<(), TextForgeError> {
        for (watcher_name, when) in self
            .schedule
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>()
        {
            match when {
                All => {
                    let return_value = {
                        let watcher_fn = self.watchers.get(watcher_name).ok_or_else(|| {
                            TextForgeError::new(
                                crate::utils::errors::TextForgeErrorCode::IndexOutOfRange(
                                    Cow::from("Watcher Not Found"),
                                ),
                                Cow::from(""),
                                Cow::from(""),
                            )
                        })?;
                        watcher_fn(input.clone())
                    };

                    self.add_to_result(self.counter, watcher_name.to_string(), return_value)?;
                }
            }
        }
        self.counter += 1;

        Ok(())
    }

    /// Serializa o relatório acumulado como JSON formatado e escreve em
    /// `filename`, sobrescrevendo o arquivo se já existir.
    ///
    /// O formato é `{ "<passo>": { "<nome_do_watcher>": "<valor>", ... }, ... }`,
    /// com as chaves de passo serializadas como string (limitação do
    /// próprio formato JSON, que não tem chaves numéricas).
    ///
    /// # Errors
    ///
    /// Retorna `Err` se a serialização falhar, ou se o arquivo não puder
    /// ser aberto/escrito.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textforge::watchers::{WatcherList, ExecutionWindows, WatcherContext};
    /// use std::env::temp_dir;
    ///
    /// let mut watchers = WatcherList::default();
    /// watchers.set_watcher("len", |ctx: WatcherContext| ctx.current.len().to_string());
    /// watchers.schedule_watcher("len", ExecutionWindows::All)?;
    ///
    /// let ctx = WatcherContext::new("banana", "", None::<&str>, "add_to_end");
    /// watchers.run_watchers(ctx)?;
    ///
    /// let path = temp_dir().join("textforge_watcherlist_doctest.json");
    /// watchers.to_json(&path)?;
    /// assert!(path.exists());
    /// # std::fs::remove_file(&path).ok();
    /// # Ok::<(), textforge::utils::errors::TextForgeError>(())
    /// ```
    pub fn to_json(&self, filename: &Path) -> Result<(), TextForgeError> {
        let json = serde_json::to_string_pretty(&self.result).map_err(|e| {
            TextForgeError::new(
                crate::utils::errors::TextForgeErrorCode::SerializationError(Cow::from(
                    e.to_string(),
                )),
                Cow::from("WatcherList.to_json"),
                Cow::from(filename.display().to_string()),
            )
        })?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)
            .map_err(|_| {
                TextForgeError::new(
                    crate::utils::errors::TextForgeErrorCode::FileOpeningError(
                        "Failed opening File".into(),
                    ),
                    "",
                    format!("{:?}", filename),
                )
            })?;

        file.write(json.as_bytes()).map_err(|_| {
            TextForgeError::new(
                crate::utils::errors::TextForgeErrorCode::FileWritingError(
                    "Failed writing text to textforge file".into(),
                ),
                "",
                "",
            )
        })?;

        Ok(())
    }
}

#[cfg(all(test, feature = "test_access"))]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;
    use std::fs;

    fn ctx(before: &str, current: &str, after: Option<&str>, instruction: &str) -> WatcherContext {
        WatcherContext::new(
            current.to_string(),
            before.to_string(),
            after.map(|s| s.to_string()),
            instruction.to_string(),
        )
    }

    #[test]
    fn test_watcher_context_new_stores_fields_verbatim() {
        let c = ctx("a", "ab", Some("abc"), "add_to_end");
        assert_eq!(c.before.to_string(), "a");
        assert_eq!(c.current.to_string(), "ab");
        assert_eq!(c.after.unwrap().to_string(), "abc".to_string());
        assert_eq!(c.instruction.to_string(), "add_to_end");
    }

    #[test]
    fn test_set_watcher_without_schedule_does_not_run() {
        let mut wl = WatcherList::default();
        wl.set_watcher("len", |c: WatcherContext| c.current.len().to_string());
        // Nunca agendado -> run_watchers não deve produzir nenhuma entrada
        wl.run_watchers(ctx("", "banana", None::<&str>, "add_to_end"))
            .unwrap();
        assert!(wl.result.get(0).is_none());
    }

    #[test]
    fn test_schedule_watcher_fails_for_unregistered_name() {
        let mut wl = WatcherList::default();
        let res = wl.schedule_watcher("nao_existe", ExecutionWindows::All);
        assert!(res.is_err());
    }

    #[test]
    fn test_schedule_watcher_succeeds_for_registered_name() {
        let mut wl = WatcherList::default();
        wl.set_watcher("len", |c: WatcherContext| c.current.len().to_string());
        assert!(wl.schedule_watcher("len", ExecutionWindows::All).is_ok());
    }

    #[test]
    fn test_run_watchers_stores_result_under_current_counter() {
        let mut wl = WatcherList::default();
        wl.set_watcher("len", |c: WatcherContext| c.current.len().to_string());
        wl.schedule_watcher("len", ExecutionWindows::All).unwrap();

        wl.run_watchers(ctx("", "banana", None::<&str>, "add_to_end"))
            .unwrap();

        let step0 = wl.result.get(0).expect("step 0 deveria existir");
        assert_eq!(step0.get("len").unwrap(), "6");
    }

    #[test]
    fn test_run_watchers_increments_counter_across_calls() {
        let mut wl = WatcherList::default();
        wl.set_watcher("len", |c: WatcherContext| c.current.len().to_string());
        wl.schedule_watcher("len", ExecutionWindows::All).unwrap();

        wl.run_watchers(ctx("", "a", None::<&str>, "add_to_end"))
            .unwrap();
        wl.run_watchers(ctx("a", "ab", None::<&str>, "add_to_end"))
            .unwrap();
        wl.run_watchers(ctx("ab", "abc", None::<&str>, "add_to_end"))
            .unwrap();

        assert_eq!(wl.result.get(0).unwrap().get("len").unwrap(), "1");
        assert_eq!(wl.result.get(1).unwrap().get("len").unwrap(), "2");
        assert_eq!(wl.result.get(2).unwrap().get("len").unwrap(), "3");
    }

    #[test]
    fn test_run_watchers_runs_multiple_scheduled_watchers_per_call() {
        let mut wl = WatcherList::default();
        wl.set_watcher("len", |c: WatcherContext| c.current.len().to_string());
        wl.set_watcher("is_empty", |c: WatcherContext| {
            c.current.is_empty().to_string()
        });
        wl.schedule_watcher("len", ExecutionWindows::All).unwrap();
        wl.schedule_watcher("is_empty", ExecutionWindows::All)
            .unwrap();

        wl.run_watchers(ctx("", "banana", None::<&str>, "add_to_end"))
            .unwrap();

        let step0 = wl.result.get(0).unwrap();
        assert_eq!(step0.get("len").unwrap(), "6");
        assert_eq!(step0.get("is_empty").unwrap(), "false");
    }

    #[test]
    fn test_reset_clears_result_and_counter_but_keeps_registration() {
        let mut wl = WatcherList::default();
        wl.set_watcher("len", |c: WatcherContext| c.current.len().to_string());
        wl.schedule_watcher("len", ExecutionWindows::All).unwrap();
        wl.run_watchers(ctx("", "banana", None::<&str>, "add_to_end"))
            .unwrap();

        wl.reset();
        assert!(wl.result.is_empty());
        assert_eq!(wl.counter, 0);

        // "len" continua registrado e agendado: run_watchers volta a produzir resultado
        // sem precisar chamar set_watcher/schedule_watcher de novo.
        wl.run_watchers(ctx("", "abcd", None::<&str>, "add_to_end"))
            .unwrap();
        assert_eq!(wl.result.get(0).unwrap().get("len").unwrap(), "4");
    }

    // NOTA: o branch de erro "Watcher Not Found" dentro de run_watchers (o segundo
    // ok_or_else, distinto do de schedule_watcher) não é alcançável pela API pública
    // atual — schedule_watcher só agenda nomes já presentes em `watchers`, e não existe
    // remove_watcher pra desincronizar os dois mapas depois. Por isso não há teste
    // exercitando esse caminho; ele é só uma checagem defensiva.

    // ... ctx() e os testes anteriores continuam iguais ...

    #[test]
    fn test_to_json_writes_valid_json_matching_result() {
        let mut wl = WatcherList::default();
        wl.set_watcher("len", |c: WatcherContext| c.current.len().to_string());
        wl.schedule_watcher("len", ExecutionWindows::All).unwrap();
        wl.run_watchers(ctx("", "banana", None::<&str>, "add_to_end"))
            .unwrap();
        wl.run_watchers(ctx("banana", "banana!", None::<&str>, "add_to_end"))
            .unwrap();

        let file = NamedTempFile::new().unwrap();
        wl.to_json(file.path()).unwrap();

        let content = fs::read_to_string(file.path()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(parsed[0]["len"], "6");
        assert_eq!(parsed[1]["len"], "7");
        // arquivo é removido automaticamente quando `file` sai de escopo
    }

    #[test]
    fn test_to_json_overwrites_existing_file() {
        let mut wl = WatcherList::default();
        wl.set_watcher("len", |c: WatcherContext| c.current.len().to_string());
        wl.schedule_watcher("len", ExecutionWindows::All).unwrap();

        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "conteudo antigo que deve ser substituido").unwrap();

        wl.run_watchers(ctx("", "ab", None::<&str>, "add_to_end"))
            .unwrap();
        wl.to_json(file.path()).unwrap();

        let content = fs::read_to_string(file.path()).unwrap();
        assert!(!content.contains("conteudo antigo"));
    }
}
